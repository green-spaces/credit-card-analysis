pub mod bill_line_description_methods;
pub mod bill_line_methods;
pub mod csv_methods;

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        Self {
            database_url: database_url.to_owned(),
        }
    }
}
