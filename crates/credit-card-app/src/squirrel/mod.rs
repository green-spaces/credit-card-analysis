use crate::{
    bill_models::{BillLineString, ParsedBillLine},
    db::Database,
    utils, Error,
};

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
}
