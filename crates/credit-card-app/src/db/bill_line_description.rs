use db_entity::entity;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use super::Database;

impl Database {
    pub async fn bld_get_all(&self) -> Vec<entity::bill_line_description::Model> {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        entity::bill_line_description::Entity::find()
            .all(&db)
            .await
            .unwrap()
    }

    pub async fn bld_create(&self, description: &str) -> entity::bill_line_description::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        // Check if it already exists
        let desc = entity::bill_line_description::Entity::find()
            .filter(entity::bill_line_description::Column::Description.eq(description.to_string()))
            .all(&db)
            .await
            .unwrap();

        // Early return
        if desc.len() == 1 {
            return desc[0].clone();
        }

        let bl_description = entity::bill_line_description::ActiveModel {
            description: Set(description.to_string()),
            ..Default::default()
        };

        bl_description.insert(&db).await.unwrap()
    }
}
