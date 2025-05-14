use std::sync::Arc;

use chrono::{Datelike, NaiveDate, Utc, Weekday};
use spdlog::{error, info};
use tauri::{AppHandle, Runtime};
use tokio::sync::RwLock;

use intelligent::article_processor::assistant::Assistant;
use intelligent::article_processor::llm_processor::{
    ArticleLLMProcessor, IPresetArticleLLMProcessor,
};
use intelligent::article_processor::melt::Melt;
use intelligent::article_processor::optimizer::Optimizer;
use intelligent::article_processor::purge::Purge;
use intelligent::article_processor::types::IArticleProcessor;
use ollama::{query_platform, Information, ProgramStatus};
use recorder::article_recorder_service::ArticleRecorderService;
use recorder::entity::article_record::{Model as ArticleRecord, Model};
use scrap::search::types::IProvider;
use scrap::search::utils::trim_html_with_script_and_style;
use scrap::search::{baidu, bing, ScrapProviderEnums};
use types::{
    AppConfig, Article, ConversationMessage, FeedTargetDescription, FeedsPackage,
    ScrapProviderType, UserConfig,
};

use crate::startup::init_app_config::sync_to;
use crate::{application_context::ApplicationContext, startup::init_user_profile};

use super::api::FeaturesAPI;

const SPECIFY_FEED_IDSET_TODAY_FILTER: &str = "TODAY_FILTER";
const SPECIFY_FEED_IDSET_WEEKEND_FILTER: &str = "WEEKEND_FILTER";

const SPECIFY_FEED_IDSET_FAVORITE_FILTER: &str = "FAVORITE_FILTER";
const SPECIFY_FEED_IDSET_UNREAD_FILTER: &str = "UNREAD_FILTER";

pub struct FeaturesAPIImpl {
    context: Arc<RwLock<ApplicationContext>>,
    scrap_provider: ScrapProviderEnums,
    article_recorder_service: ArticleRecorderService,
}

impl FeaturesAPIImpl {
    // 创建FeaturesAPIImpl实例
    pub async fn new(ctx: ApplicationContext) -> anyhow::Result<Self> {
        // 基础配置信息
        let app_config = &ctx.app_config;
        let llm_section = app_config.llm.clone();

        // 初始化ArticleRecorderService实例
        let mut article_recorder_service = ArticleRecorderService::default();
        article_recorder_service.initialize().await?;

        // 初始化爬虫实例
        let scrap_provider = &app_config.scrap.provider;
        let scrap_provider = match scrap_provider {
            ScrapProviderType::Baidu => {
                ScrapProviderEnums::Baidu(baidu::Provider::new(llm_section)?)
            }
            ScrapProviderType::Bing => ScrapProviderEnums::Bing(bing::Provider::new(llm_section)?),
        };
        let context = Arc::new(RwLock::new(ctx));

        Ok(FeaturesAPIImpl {
            context,
            scrap_provider,
            article_recorder_service,
        })
    }

    async fn sync_user_profile(&self, user_config: &UserConfig) -> anyhow::Result<()> {
        init_user_profile::sync_to(user_config).await
    }

    async fn process_article_pipelines(
        &self,
        article: &mut Article,
        purge: &ArticleLLMProcessor,
        optimizer: &ArticleLLMProcessor,
        melt: &ArticleLLMProcessor,
    ) -> anyhow::Result<(Article, Article, Article)> {
        let out_purged_article = purge.process(article).await?;
        info!(
            "article purged, title = {}, source_link = {}, optimizing",
            article.title, article.source_link
        );

        let out_optimized_article = optimizer.process(&out_purged_article).await?;
        info!(
            "purged article optimized, title = {}, melting",
            out_purged_article.title
        );
        if let Some(optimized_content) = out_optimized_article.content.clone() {
            if optimized_content.contains("QINO-AGENTIC-EXECUTION-FAILURE") {
                return Err(anyhow::Error::msg("QINO-AGENTIC-EXECUTION-FAILURE"));
            }
        }

        let out_melted_article = melt.process(&out_optimized_article).await?;
        info!(
            "optimized article melted, title = {}, recording",
            out_melted_article.title
        );

        Ok((
            out_purged_article,
            out_optimized_article,
            out_melted_article,
        ))
    }
}

// 在 FeaturesAPIImpl 结构体上实现 FeaturesAPI 接口的所有方法
impl FeaturesAPI for FeaturesAPIImpl {
    // #[macro_api_interceptor::monitor]
    async fn add_feeds_package(&self, feeds_package: FeedsPackage) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        if user_config.add_feeds_packages(feeds_package) {
            return self.sync_user_profile(user_config).await;
        }
        Err(anyhow::Error::msg(
            "add_feeds_package failure, may be the feeds package already existed",
        ))
    }

    async fn remove_feeds_package(&self, package_id: &str) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        if user_config.remove_feeds_package(package_id) {
            return self.sync_user_profile(user_config).await;
        }
        Err(anyhow::Error::msg(
            format!(
                "remove_feeds_package failure, the feeds package was not found, the requirements of package_id = {}",
                package_id,
            )
        ))
    }

    async fn rename_feeds_package(&self, package_id: &str, new_name: &str) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        if user_config.rename_feeds_package(package_id, new_name) {
            self.sync_user_profile(user_config).await?;
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "rename_feeds_package failure, the feeds package was not found, the requirements of package_id = {}",
                package_id,
            )))
        }
    }

    async fn add_feed(&self, package_id: &str, ftd: FeedTargetDescription) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        match user_config.find_feeds_package_mut(package_id) {
            Some(package) => {
                package.feeds.push(ftd);
                self.sync_user_profile(user_config).await
            }
            None => Err(anyhow::Error::msg(
                format!(
                    "add_feed failure, the feeds package was not found, the requirements of package_id = {}, feed_name = {}, feed_id = {}",
                    package_id,
                    ftd.name.as_str(),
                    ftd.id.as_str()
                )
            ))
        }
    }

    async fn remove_feed(&self, package_id: &str, feed_id: &str) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        match user_config.remove_feed(package_id, feed_id) {
            true => self.sync_user_profile(user_config).await,
            false => Err(anyhow::Error::msg(
                format!(
                    "remove_feed failure, the feeds package was not found, the requirements of package_id = {}, feed_id = {}",
                    package_id,
                    feed_id
                )
            ))
        }
    }

    async fn rename_feed(
        &self,
        package_id: &str,
        feed_id: &str,
        new_name: &str,
    ) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        match user_config.rename_feed(package_id, feed_id, new_name) {
            true => self.sync_user_profile(user_config).await,
            false => Err(anyhow::Error::msg(
                format!(
                    "rename_feed failure, the feeds package was not found, the requirements of package_id = {}, feed_id = {}",
                    package_id,
                    feed_id
                )
            ))
        }
    }

    async fn change_feed_data(
        &self,
        package_id: &str,
        feed_id: &str,
        data: Vec<String>,
    ) -> anyhow::Result<()> {
        let context_guarded = &mut self.context.write().await;
        let user_config = &mut context_guarded.user_config;
        match user_config.change_feed_data(package_id, feed_id, data) {
            true => self.sync_user_profile(user_config).await,
            false => Err(anyhow::Error::msg(
                format!(
                    "change_feed_data failure, the feeds package was not found, the requirements of package_id = {}, feed_id = {}",
                    package_id,
                    feed_id
                )
            ))
        }
    }

    async fn get_feeds_packages(&self) -> Vec<FeedsPackage> {
        let context_guarded = &self.context.read().await;
        context_guarded.user_config.feeds_packages.clone()
    }

    async fn get_feeds_by_package(&self, package_id: &str) -> Option<FeedsPackage> {
        let context_guarded = &self.context.read().await;
        context_guarded.user_config.find_feeds_package(package_id)
    }

    async fn update_feed_contents<R: Runtime>(
        &self,
        package_id: &str,
        feed_id: &str,
        app_handle: Option<AppHandle<R>>,
    ) -> anyhow::Result<()> {
        let user_config;
        let llm_section;
        {
            let context_guarded = self.context.read().await;
            user_config = context_guarded.user_config.clone();
            llm_section = context_guarded.app_config.llm.clone();
        }
        if let Some(ftd) = user_config.find_feed(package_id, feed_id) {
            let words: Vec<&str> = ftd.data.iter().map(|x| x.as_str()).collect();
            info!("scraping, via the words...{:?}", words);
            let mut articles = self
                .scrap_provider
                .search_by_words(words, app_handle)
                .await?;
            info!(
                "found {} articles for the feed_id = {}, feed_name = {}",
                articles.len(),
                feed_id,
                ftd.name
            );
            let article_recorder_service = &self.article_recorder_service;
            let purge = Purge::new_processor(llm_section.clone())?;
            let optimizer = Optimizer::new_processor(llm_section.clone())?;
            let melt = Melt::new_processor(llm_section.clone())?;
            for article in articles.iter_mut() {
                if article_recorder_service
                    .exists_by_source(&article.source_link)
                    .await?
                {
                    info!(
                        "article existed, title = {}, source_link = {}",
                        article.title, article.source_link
                    );
                    continue;
                }
                match self.process_article_pipelines(article, &purge, &optimizer, &melt).await {
                    Ok((out_purged_article, out_optimized_article, out_melted_article)) => {
                        article_recorder_service
                            .insert(vec![ArticleRecord {
                                id: 0,
                                source_link: out_melted_article.source_link,
                                title: out_melted_article.title,
                                purged_content: out_purged_article.content.unwrap_or_default(),
                                head_read: out_purged_article.head_read.unwrap_or_default(),
                                optimized_content: out_optimized_article.content.unwrap_or_default(),
                                melted_content: out_melted_article.content.unwrap_or_default(),
                                published_at: Utc::now().date_naive(),
                                created_at: Utc::now().date_naive(),
                                has_read: false,
                                is_favorite: false,
                                group_id: feed_id.into(),
                            }])
                            .await?;
                        info!(
                    "article recorded to the feed_name = {}, title = {}",
                    ftd.name, article.title
                );
                    }
                    Err(e) => error!(
                        "process_article_pipelines execution failure, the requirements of package_id = {}, feed_id = {}, source_link = {}, error = {}",
                        package_id,
                        feed_id,
                        article.source_link,
                        e)
                };
            }
            return Ok(());
        }
        let error_msg = format!(
            "update_feed_contents failure, the feed was not found, the requirements of package_id = {}, feed_id = {}",
            package_id,
            feed_id
        );
        error!("{}", error_msg);
        Err(anyhow::Error::msg(error_msg))
    }

    /// 读取订阅中的内容，支持分页。
    async fn read_feed_contents(
        &self,
        feed_id: &str,
        offset: u64,
        count: u64,
    ) -> anyhow::Result<Vec<Model>> {
        if feed_id == SPECIFY_FEED_IDSET_TODAY_FILTER {
            let now: NaiveDate = Utc::now().date_naive();
            return self
                .article_recorder_service
                .query_backward_in_duration(offset, count, now, now)
                .await;
        }
        if feed_id == SPECIFY_FEED_IDSET_WEEKEND_FILTER {
            let end: NaiveDate = Utc::now().date_naive();
            let monday_days_from_now = ((end.weekday() as i64 - Weekday::Mon as i64) % 7) as u64;
            let start = end
                .checked_sub_days(chrono::Days::new(monday_days_from_now))
                .expect("SPECIFY_FEED_IDSET_WEEKEND_FILTER：日期转换错误，找不到本周周一的日期。");
            return self
                .article_recorder_service
                .query_backward_in_duration(offset, count, start, end)
                .await;
        }
        if feed_id == SPECIFY_FEED_IDSET_FAVORITE_FILTER {
            return self
                .article_recorder_service
                .query_favorite(offset, count)
                .await;
        }
        if feed_id == SPECIFY_FEED_IDSET_UNREAD_FILTER {
            return self
                .article_recorder_service
                .query_unread(offset, count)
                .await;
        }
        self.article_recorder_service
            .query_backward(Some(feed_id), offset, count)
            .await
    }

    async fn query_by_id(&self, id: i32) -> anyhow::Result<Option<Model>> {
        self.article_recorder_service.query_by_id(id).await
    }

    async fn mark_as_read(&self, id: i32) -> anyhow::Result<()> {
        self.article_recorder_service.mark_as_read(id).await?;
        Ok(())
    }

    async fn set_favorite(&self, id: i32, is_favorite: bool) -> anyhow::Result<()> {
        self.article_recorder_service
            .set_favorite(id, is_favorite)
            .await?;
        Ok(())
    }

    async fn get_app_config(&self) -> anyhow::Result<AppConfig> {
        let context_guarded = self.context.read().await;
        Ok(context_guarded.app_config.clone())
    }

    async fn set_app_config(&self, app_config: AppConfig) -> anyhow::Result<()> {
        let mut context_guarded = self.context.write().await;
        context_guarded.app_config = app_config;
        sync_to(&context_guarded.app_config).await?;
        Ok(())
    }

    /// 获得Ollama实例状态
    async fn get_ollama_status(&self) -> anyhow::Result<ProgramStatus> {
        let context_guarded = &self.context.read().await;
        let llm_section = &context_guarded.app_config.llm.provider_ollama;
        match query_platform(&llm_section.endpoint).await {
            Ok(information) => Ok(information.status),
            Err(_) => Ok(ProgramStatus::Uninstall),
        }
    }

    /// 下载Ollama
    async fn download_ollama(&self) -> anyhow::Result<()> {
        open::that("https://ollama.com/download")?;
        Ok(())
    }

    /// 启动Ollama实例
    async fn launch_ollama(&self) -> anyhow::Result<()> {
        ollama::launch().await
    }

    async fn open_article_external(&self, url: &str) -> anyhow::Result<()> {
        open::that(url)?;
        Ok(())
    }

    async fn update_article_by_source(
        &self,
        article_id: i32,
        source_text: String,
    ) -> anyhow::Result<bool> {
        let sharked_html = trim_html_with_script_and_style(source_text.as_str());
        let context_guarded = &self.context.read().await;
        let llm_section = &context_guarded.app_config.llm;

        let article_recorder_service = &self.article_recorder_service;
        let purge = Purge::new_processor(llm_section.clone())?;
        let optimizer = Optimizer::new_processor(llm_section.clone())?;
        let melt = Melt::new_processor(llm_section.clone())?;

        let article_opt = article_recorder_service.query_by_id(article_id).await?;
        match article_opt {
            None => Ok(false),
            Some(article_model) => {
                let mut article = Article {
                    title: article_model.title.to_owned(),
                    head_read: Some(article_model.head_read.to_owned()),
                    source_link: article_model.source_link.to_owned(),
                    summary: None,
                    content: Some(sharked_html),
                    date_created: "".to_string(),
                    date_read: None,
                };
                match self
                    .process_article_pipelines(&mut article, &purge, &optimizer, &melt)
                    .await
                {
                    Ok((out_purged_article, out_optimized_article, out_melted_article)) => {
                        let purged_content = out_purged_article.content.unwrap_or_default();
                        let optimized_content = out_optimized_article.content.unwrap_or_default();
                        let melted_content = out_melted_article.content.unwrap_or_default();
                        article_recorder_service
                            .update_content(
                                article_model,
                                purged_content,
                                optimized_content,
                                melted_content,
                            )
                            .await?;
                        info!("article updated, article_id = {}", article_id);
                        Ok(true)
                    }
                    Err(e) => {
                        error!(
                        "process_article_pipelines execution failure, the requirements of article_id = {}, error = {}",
                        article_id,
                        e);
                        Ok(false)
                    }
                }
            }
        }
    }

    async fn chat_with_article_assistant(
        &self,
        article_id: i32,
        user_prompt: &str,
        history: Vec<ConversationMessage>,
    ) -> anyhow::Result<String> {
        let context_guarded = &self.context.read().await;
        let llm_section = &context_guarded.app_config.llm;
        let assistant = Assistant::new(llm_section.clone());

        let article_opt = self
            .article_recorder_service
            .query_by_id(article_id)
            .await?;
        if let Some(article) = article_opt {
            return assistant
                .chat(article.optimized_content, user_prompt, history)
                .await;
        }
        Ok(format!(
            "文章不存在, article_id = {}, user_prompt = {}",
            article_id, user_prompt
        ))
    }

    async fn search_contents_by_keyword(
        &self,
        keyword: &str,
        offset: u64,
        count: u64,
    ) -> anyhow::Result<Vec<Model>> {
        self.article_recorder_service
            .search_contents_by_keyword(keyword, offset, count)
            .await
    }
}
