use db_entity::entity;
use sea_orm::{ActiveModelTrait, Set};

use super::Database;

impl Database {
    pub async fn insert_csv(&self, text: String) -> entity::raw_csv::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        let raw_csv_model = entity::raw_csv::ActiveModel {
            text: Set(text),
            ..Default::default()
        };

        raw_csv_model.insert(&db).await.unwrap()
    }
}
