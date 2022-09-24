use db_entity::entity::{self, bill_line_description};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use super::Database;

impl Database {
    pub async fn bld_read(&self, id: i32) -> db_entity::entity::bill_line_description::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        entity::bill_line_description::Entity::find_by_id(id)
            .one(&db)
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn bld_read_all(&self) -> Vec<entity::bill_line_description::Model> {
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
            .one(&db)
            .await
            .unwrap();

        // Early return
        if desc.is_some() {
            return desc.unwrap();
        }

        let bl_description = entity::bill_line_description::ActiveModel {
            description: Set(description.to_string()),
            ..Default::default()
        };

        bl_description.insert(&db).await.unwrap()
    }

    pub async fn bld_update(
        &self,
        bld_model: bill_line_description::ActiveModel,
    ) -> bill_line_description::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();
        let model = bld_model.update(&db).await.unwrap();
        model
    }
}
