use std::time::Duration;

use sea_orm::{ActiveModelTrait, ConnectionTrait, ConnectOptions, Database, DatabaseConnection, DbBackend, DeleteResult, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Schema};
use sea_orm::sea_query::IntoCondition;

use crate::entity::article_record;
use crate::entity::article_record::Column;
use crate::path::get_appdata_articles;

pub struct Operator {
    db: Option<DatabaseConnection>,
}

impl Operator {
    pub fn new() -> Operator {
        Operator { db: None }
    }

    fn ensure_db_initialized(&self) -> &DatabaseConnection {
        match &self.db {
            None => panic!("database connection not initialized"),
            Some(db_inst) => db_inst,
        }
    }

    pub async fn create_table_if_not_existed(&self) -> anyhow::Result<()> {
        let db = self.ensure_db_initialized();
        if article_record::Entity::find().count(db).await.is_ok() {
            return Ok(());
        }
        let db_sqlite = DbBackend::Sqlite;
        let schema = Schema::new(db_sqlite);
        let statement_create =
            db_sqlite.build(&schema.create_table_from_entity(article_record::Entity));
        db.execute(statement_create).await?;
        Ok(())
    }

    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        let mut opt = ConnectOptions::new(format!(
            "sqlite://{}?mode=rwc",
            get_appdata_articles().to_str().expect("获取Recorder数据库文件路径失败")
        ));
        opt.max_connections(10)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(16))
            .max_lifetime(Duration::from_secs(16))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        self.db = Some(Database::connect(opt).await?);
        self.create_table_if_not_existed().await?;
        Ok(())
    }

    pub async fn count(&self) -> anyhow::Result<u64> {
        let db = self.ensure_db_initialized();
        let count = article_record::Entity::find().count(db).await?;
        Ok(count)
    }

    pub async fn dispose(&mut self) -> anyhow::Result<()> {
        let disposing_db = self.db.take();
        match disposing_db {
            Some(db) => {
                db.close().await?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub async fn insert(&self, entity: article_record::ActiveModel) -> anyhow::Result<()> {
        entity.insert(self.ensure_db_initialized()).await?;
        Ok(())
    }

    pub async fn update(
        &self,
        entity: article_record::ActiveModel,
    ) -> anyhow::Result<article_record::Model> {
        Ok(entity.update(self.ensure_db_initialized()).await?)
    }

    pub async fn query<F>(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
        filter: F,
    ) -> anyhow::Result<Vec<article_record::Model>>
        where
            F: IntoCondition,
    {
        Ok(article_record::Entity::find()
            .filter(filter)
            .order_by_desc(Column::PublishedAt)
            .order_by_desc(Column::Id)
            .offset(offset)
            .limit(limit)
            .all(self.ensure_db_initialized())
            .await?)
    }

    pub async fn query_by_filters<F>(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
        filters: Vec<F>,
    ) -> anyhow::Result<Vec<article_record::Model>>
        where
            F: IntoCondition,
    {
        let mut finder = article_record::Entity::find();
        for filter in filters.into_iter() {
            finder = finder.filter(filter);
        }
        Ok(finder
            .order_by_desc(Column::PublishedAt)
            .order_by_desc(Column::Id)
            .offset(offset)
            .limit(limit)
            .all(self.ensure_db_initialized())
            .await?)
    }

    pub async fn exists<F>(&self, filter: F) -> anyhow::Result<bool>
        where F: IntoCondition {
        let count = article_record::Entity::find().filter(filter).count(self.ensure_db_initialized()).await?;
        Ok(count > 0)
    }

    pub async fn query_without_filter(
        &self,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> anyhow::Result<Vec<article_record::Model>> {
        Ok(article_record::Entity::find()
            .offset(offset)
            .limit(limit)
            .order_by_desc(Column::PublishedAt)
            .order_by_desc(Column::Id)
            .all(self.ensure_db_initialized())
            .await?)
    }

    pub async fn query_by_id(&self, id: i32) -> anyhow::Result<Option<article_record::Model>> {
        Ok(article_record::Entity::find_by_id(id)
            .one(self.ensure_db_initialized())
            .await?)
    }

    pub async fn delete(
        &self,
        entity: article_record::ActiveModel,
    ) -> anyhow::Result<DeleteResult> {
        Ok(entity.delete(self.ensure_db_initialized()).await?)
    }
}
