use ollama::ProgramStatus;
use recorder::entity::article_record::Model;
use tauri::{AppHandle, Runtime};
use types::{AppConfig, ConversationMessage, FeedTargetDescription, FeedsPackage};

/// 功能模块的门面API定义
pub trait FeaturesAPI {
    /// 用于添加订阅包，会同步到用户配置存储模块
    /// 如果订阅包已经存在，函数将返回一个错误
    fn add_feeds_package(
        &self,
        feeds_package: FeedsPackage,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 用于删除订阅包，会同步到用户配置存储模块
    /// 如果订阅包不存在，函数将返回一个错误
    fn remove_feeds_package(
        &self,
        package_id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 用于重命名订阅包，会同步到用户配置存储模块
    /// 如果订阅包不存在，函数将返回一个错误
    fn rename_feeds_package(
        &self,
        package_id: &str,
        new_name: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 用于添加订阅到订阅包，会同步到用户配置存储模块
    /// 如果订阅已经存在，函数将返回一个错误
    fn add_feed(
        &self,
        package_id: &str,
        ftd: FeedTargetDescription,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 用于移除订阅包中的订阅，会同步到用户配置存储模块
    /// 如果订阅不存在，函数将返回一个错误
    fn remove_feed(
        &self,
        package_id: &str,
        feed_id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 用于重命名订阅包中的订阅，会同步到用户配置存储模块
    /// 如果订阅不存在，函数将返回一个错误
    fn rename_feed(
        &self,
        package_id: &str,
        feed_id: &str,
        new_name: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;
    fn change_feed_data(
        &self,
        package_id: &str,
        feed_id: &str,
        data: Vec<String>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 获得所有的订阅包信息
    fn get_feeds_packages(&self) -> impl std::future::Future<Output = Vec<FeedsPackage>>;

    /// 获得制定订阅包中的订阅信息
    fn get_feeds_by_package(
        &self,
        package_id: &str,
    ) -> impl std::future::Future<Output = Option<FeedsPackage>>;

    /// 更新订阅内容，将爬取数据源并做内容提取和总结、同步到数据存储模块。
    fn update_feed_contents<R: Runtime>(
        &self,
        package_id: &str,
        feed_id: &str,
        app_handle: Option<AppHandle<R>>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn read_feed_contents(
        &self,
        feed_id: &str,
        offset: u64,
        count: u64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Model>>>;

    fn query_by_id(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Model>>>;
    fn mark_as_read(&self, id: i32) -> impl std::future::Future<Output = anyhow::Result<()>>;
    fn set_favorite(
        &self,
        id: i32,
        is_favorite: bool,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    /// 读取AppConfig
    fn get_app_config(&self) -> impl std::future::Future<Output = anyhow::Result<AppConfig>>;

    /// 覆盖存储AppConfig
    fn set_app_config(
        &self,
        app_config: AppConfig,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn get_ollama_status(&self)
        -> impl std::future::Future<Output = anyhow::Result<ProgramStatus>>;

    fn download_ollama(&self) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn launch_ollama(&self) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn open_article_external(
        &self,
        url: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn update_article_by_source(
        &self,
        article_id: i32,
        source_text: String,
    ) -> impl std::future::Future<Output = anyhow::Result<bool>>;

    fn chat_with_article_assistant(
        &self,
        article_id: i32,
        user_prompt: &str,
        history: Vec<ConversationMessage>,
    ) -> impl std::future::Future<Output = anyhow::Result<String>>;

    fn search_contents_by_keyword(
        &self,
        keyword: &str,
        offset: u64,
        count: u64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Model>>>;
}
