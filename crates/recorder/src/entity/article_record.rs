use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// 源数据、提取的原正文、优化的正文、导读内容、新闻日期、创建日期、是否已阅读
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "t_article_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub source_link: String,
    pub title: String,
    pub purged_content: String,
    pub head_read: String,
    pub optimized_content: String,
    pub melted_content: String,
    #[sea_orm(column_type = "Date")]
    pub published_at: chrono::NaiveDate,
    #[sea_orm(column_type = "Date")]
    pub created_at: chrono::NaiveDate,
    pub has_read: bool,
    pub is_favorite: bool,
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
