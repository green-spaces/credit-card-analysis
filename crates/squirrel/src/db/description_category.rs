use db_entity::entity::description_category;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

use super::Database;

impl Database {
    // Creates the description category or returns one if it already exists
    pub async fn description_category_create(
        &self,
        name: &str,
    ) -> Result<description_category::Model, DbErr> {
        if let Some(model) = self.description_category_read_by_name(name).await? {
            return Ok(model);
        }

        let db = sea_orm::Database::connect(&self.database_url).await?;

        let dc = description_category::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        };

        dc.insert(&db).await
    }

    pub async fn description_category_read(
        &self,
        id: i32,
    ) -> Result<Option<description_category::Model>, DbErr> {
        let db = sea_orm::Database::connect(&self.database_url).await?;

        description_category::Entity::find_by_id(id).one(&db).await
    }

    pub async fn description_category_read_by_name(
        &self,
        name: &str,
    ) -> Result<Option<description_category::Model>, DbErr> {
        let db = sea_orm::Database::connect(&self.database_url).await?;

        description_category::Entity::find()
            .filter(description_category::Column::Name.eq(name))
            .one(&db)
            .await
    }

    pub async fn description_category_read_all(
        &self,
    ) -> Result<Vec<description_category::Model>, DbErr> {
        let db = sea_orm::Database::connect(&self.database_url).await?;

        description_category::Entity::find().all(&db).await
    }

    pub async fn all_dc_and_bld(
        &self,
    ) -> Result<
        Vec<(
            db_entity::entity::description_category::Model,
            Vec<db_entity::entity::bill_line_description::Model>,
        )>,
        DbErr,
    > {
        let db = sea_orm::Database::connect(&self.database_url).await?;

        description_category::Entity::find()
            .find_with_related(db_entity::entity::bill_line_description::Entity)
            .all(&db)
            .await
    }
}
