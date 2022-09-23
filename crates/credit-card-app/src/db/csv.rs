use db_entity::entity::raw_csv;
use sea_orm::{ActiveModelTrait, Set};

use super::Database;
use crate::Error;

impl Database {
    pub async fn csv_create(&self, text: String) -> Result<raw_csv::Model, Error> {
        let db = sea_orm::Database::connect(&self.database_url).await?;

        let raw_csv_model = raw_csv::ActiveModel {
            text: Set(text),
            ..Default::default()
        };

        Ok(raw_csv_model.insert(&db).await?)
    }
}
