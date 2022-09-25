pub mod bill_line;
pub mod bill_line_description;
pub mod csv;
pub mod description_category;

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
