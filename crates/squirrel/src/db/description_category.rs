use db_entity::entity::description_category;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Set};

use super::Database;

impl Database {
    pub async fn description_category_create(
        &self,
        name: &str,
    ) -> Result<description_category::Model, DbErr> {
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
}
