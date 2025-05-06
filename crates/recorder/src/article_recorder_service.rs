use chrono::NaiveDate;
use sea_orm::{ColumnTrait, Condition, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use crate::entity::article_record;
use crate::entity::article_record::Model;
use crate::operator::Operator;

pub struct ArticleRecorderService {
    operator: Operator,
}

impl Default for ArticleRecorderService {
    fn default() -> Self {
        let operator = Operator::new();
        ArticleRecorderService { operator }
    }
}

impl ArticleRecorderService {
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        self.operator.initialize().await
    }

    pub async fn update_content(&self, record: Model, purged_content: String, optimized_content: String, melted_content: String) -> anyhow::Result<Model> {
        let operator = &self.operator;
        let mut active_model = record.into_active_model();
        active_model.purged_content = Set(purged_content);
        active_model.optimized_content = Set(optimized_content);
        active_model.melted_content = Set(melted_content);
        operator.update(active_model).await
    }

    pub async fn insert(&self, records: Vec<Model>) -> anyhow::Result<i32> {
        // 剔除掉已有的且未读过的，插入掉其他情况的。
        let mut inserted_num = 0;
        let operator = &self.operator;
        for record in records {
            let duplicates: Vec<Model> = operator
                .query(
                    None,
                    None,
                    article_record::Column::SourceLink.eq(&record.source_link),
                )
                .await?;
            let mut has_existed_unread = false;
            for duplicate in duplicates {
                if duplicate.has_read {
                    operator.delete(duplicate.into_active_model()).await?;
                } else {
                    has_existed_unread = true;
                }
            }
            if !has_existed_unread {
                let mut active_model = record.into_active_model();
                active_model.id = Default::default();
                operator.insert(active_model).await?;
                inserted_num += 1;
            }
        }
        Ok(inserted_num)
    }

    pub async fn exists_by_source(&self, source_link: &String) -> anyhow::Result<bool> {
        let operator = &self.operator;
        operator.exists(article_record::Column::SourceLink.eq(source_link)).await
    }

    pub async fn query_backward(&self, group_id: Option<&str>, offset: u64, max_count: u64) -> anyhow::Result<Vec<Model>> {
        let operator = &self.operator;
        match group_id {
            None => operator
                .query_without_filter(Some(offset), Some(max_count))
                .await,
            Some(group_id) => operator
                .query(Some(offset), Some(max_count), article_record::Column::GroupId.eq(group_id))
                .await
        }
    }

    pub async fn query_favorite(&self, offset: u64, max_count: u64) -> anyhow::Result<Vec<Model>> {
        let operator = &self.operator;
        operator
            .query(Some(offset), Some(max_count), article_record::Column::IsFavorite.eq(true))
            .await
    }

    pub async fn query_unread(&self, offset: u64, max_count: u64) -> anyhow::Result<Vec<Model>> {
        let operator = &self.operator;
        let filters = vec![article_record::Column::HasRead.eq(false)];
        operator
            .query_by_filters(Some(offset), Some(max_count), filters)
            .await
    }

    pub async fn query_backward_in_duration(&self, offset: u64, max_count: u64, begin: NaiveDate, end: NaiveDate) -> anyhow::Result<Vec<Model>> {
        let operator = &self.operator;
        operator
            .query(Some(offset), Some(max_count), article_record::Column::PublishedAt.between(begin, end))
            .await
    }

    pub async fn count(&self) -> anyhow::Result<u64> {
        let operator = &self.operator;
        operator.count().await
    }

    pub async fn mark_as_read(&self, id: i32) -> anyhow::Result<Option<Model>> {
        let operator = &self.operator;
        let record = operator.query_by_id(id).await?;
        match record {
            None => Ok(None),
            Some(record) => {
                let mut active_model = record.into_active_model();
                active_model.has_read = Set(true);
                let updated_record = operator.update(active_model).await?;
                Ok(Some(updated_record))
            }
        }
    }

    pub async fn set_favorite(&self, id: i32, is_favorite: bool) -> anyhow::Result<Option<Model>> {
        let operator = &self.operator;
        let record = operator.query_by_id(id).await?;
        match record {
            None => Ok(None),
            Some(record) => {
                let mut active_model = record.into_active_model();
                active_model.is_favorite = Set(is_favorite);
                let updated_record = operator.update(active_model).await?;
                Ok(Some(updated_record))
            }
        }
    }

    pub async fn query_by_id(&self, id: i32) -> anyhow::Result<Option<Model>> {
        self.operator.query_by_id(id).await
    }

    pub async fn dispose(&mut self) -> anyhow::Result<()> {
        self.operator.dispose().await
    }

    pub async fn search_contents_by_keyword(&self, keyword: &str, offset: u64, max_count: u64) -> anyhow::Result<Vec<Model>> {
        let operator = &self.operator;
        let keyword = format!("%{}%", keyword.to_lowercase());
        operator
            .query(Some(offset), Some(max_count),
                   Condition::any()
                       .add(Expr::cust_with_values("LOWER(title) LIKE?", vec![keyword.clone()]))
                       .add(Expr::cust_with_values("LOWER(head_read) LIKE?", vec![keyword.clone()]))
                       .add(Expr::cust_with_values("LOWER(melted_content) LIKE?", vec![keyword.clone()])),
            )
            .await
    }
}
