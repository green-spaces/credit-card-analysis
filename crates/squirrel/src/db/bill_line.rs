use db_entity::entity;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, Set};

use crate::models::bill_models::ParsedBillLine;

use super::Database;

impl Database {
    pub async fn bill_line_create(
        &self,
        bill_line: ParsedBillLine,
        csv_id: i32,
    ) -> entity::bill_line::Model {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        let des_model = self.bld_create(&bill_line.description).await;

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

    pub async fn bill_lines_for_bld(
        &self,
        bld: &entity::bill_line_description::Model,
    ) -> Vec<entity::bill_line::Model> {
        let db = sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap();

        entity::bill_line::Entity::find()
            .filter(entity::bill_line::Column::DescriptionId.eq(bld.id))
            .all(&db)
            .await
            .unwrap()
    }
}
