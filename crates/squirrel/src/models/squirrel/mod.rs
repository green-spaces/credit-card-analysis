use futures::future::join_all;
use sea_orm::Set;

use db_entity::entity::bill_line_description::{self, Model};

use super::bill_models::{BillLineString, ParsedBillLine};
use crate::{db::Database, utils, Error};

/// Core functionally for the Squirrel application
pub struct Squirrel {
    db: Database,
}

impl Squirrel {
    pub fn new(database_url: &str) -> Self {
        Self {
            db: Database::new(database_url),
        }
    }
}

impl Squirrel {
    /// Parses and loads a csv into the database
    pub async fn load_csv(&self, file_path: &str) -> Result<(), Error> {
        let csv_contents = utils::read_file_to_string(file_path);
        let csv_model = self.db.csv_create(csv_contents.clone()).await?;

        let bill_lines = BillLineString::parse_csv(csv_contents)
            .into_iter()
            .filter_map(|i| ParsedBillLine::try_from(i).ok())
            .collect::<Vec<ParsedBillLine>>();

        for line in bill_lines {
            let _model = self.db.bill_line_create(line, csv_model.id).await;
        }

        Ok(())
    }

    /// Returns all the [BillLineDescription]s that do not have a [CategoryDescription]
    pub async fn bld_not_categorized(&self) -> Vec<Model> {
        self.db
            .bld_read_all()
            .await
            .into_iter()
            .filter(|bld| bld.description_category_id.is_none())
            .collect::<Vec<_>>()
    }

    /// Create a new [DescriptionCategory] and returns it's id
    ///
    /// If the [DescriptionCategory] already exists, its id is returned
    pub async fn dc_create(&self, name: &str) -> i32 {
        let model = self.db.description_category_create(name).await.unwrap();
        model.id
    }

    pub async fn map_dc_to_bld(&self, dc_id: i32, bld_id: i32) -> Result<(), Error> {
        let bld = self.db.bld_read(bld_id).await;
        let mut bld_active: bill_line_description::ActiveModel = bld.into();
        bld_active.description_category_id = Set(Some(dc_id));
        self.db.bld_update(bld_active).await;
        Ok(())
    }

    /// Returns all the [DescriptionCategory]s stored in the database
    pub async fn all_dc(&self) -> Vec<db_entity::entity::description_category::Model> {
        self.db.description_category_read_all().await.unwrap()
    }

    /// Returns all the [DescriptionCategory]s and the BillLines associated to them
    pub async fn all_dc_and_bl(
        &self,
    ) -> Vec<(
        db_entity::entity::description_category::Model,
        Vec<db_entity::entity::bill_line::Model>,
    )> {
        let bld = self.db.all_dc_and_bld().await.unwrap();
        let mut bl = Vec::new();

        for (category, blds) in bld.iter() {
            // let mut single_bls = Vec::new();
            // for bld in blds {
            //     let bill_lines = self.db.bill_lines_for_bld(bld).await;
            //     single_bls.extend(bill_lines);
            // }

            let single_bls = join_all(blds.iter().map(|bld| self.db.bill_lines_for_bld(bld)))
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            bl.push((category.clone(), single_bls));
        }
        bl
    }
}
