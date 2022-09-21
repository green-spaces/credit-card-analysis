use db_entity::entity;
use sea_orm::{ActiveModelTrait, NotSet, Set};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::bill_models::BillLine;

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.to_owned(),
        }
    }

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

    pub async fn insert_bill_line_description(
        &self,
        bill_line_desc: &str,
    ) -> entity::bill_line_description::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        // Check if it already exists
        let desc = entity::bill_line_description::Entity::find()
            .filter(
                entity::bill_line_description::Column::Description.eq(bill_line_desc.to_string()),
            )
            .all(&db)
            .await
            .unwrap();

        // Early return
        if desc.len() == 1 {
            return desc[0].clone();
        }

        let bl_description = entity::bill_line_description::ActiveModel {
            description: Set(bill_line_desc.to_string()),
            ..Default::default()
        };

        bl_description.insert(&db).await.unwrap()
    }

    pub async fn insert_bill_line(
        &self,
        bill_line: BillLine,
        csv_id: i32,
    ) -> entity::bill_line::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        let des_model = self
            .insert_bill_line_description(&bill_line.description)
            .await;

        println!("Desc: {:#?}", des_model);

        let bl = entity::bill_line::ActiveModel {
            transaction_data: Set(bill_line.transaction_date.clone().into()),
            description_id: Set(des_model.id),
            debit: bill_line
                .debit
                .map(|v| Set(Some(v as f64)))
                .unwrap_or(NotSet),
            credit: bill_line
                .credit
                .map(|v| Set(Some(v as f64)))
                .unwrap_or(NotSet),
            balance: Set(bill_line.balance as f64),
            raw_csv_id: Set(csv_id),
            ..Default::default()
        };

        bl.insert(&db).await.unwrap()
    }
}
