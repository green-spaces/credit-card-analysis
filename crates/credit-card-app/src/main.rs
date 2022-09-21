use credit_card_app::{
    bill_models::{BillLine, BillLineString},
    utils,
};

use credit_card_app::db;

#[tokio::main]
async fn main() {
    let db = db::Database::new("sqlite://test.db");

    let sample = "./bills/July-2022.csv";

    let csv_contents = utils::read_file_to_string(sample);
    let csv_model = db.insert_csv(csv_contents.clone()).await;

    println!("{:#?}", csv_model);

    let bill_lines = BillLineString::parse_csv(csv_contents)
        .into_iter()
        .filter_map(|i| BillLine::try_from(i).ok())
        .collect::<Vec<BillLine>>();

    for line in bill_lines {
        println!("{:?}", line);
        let model = db.insert_bill_line(line, csv_model.id).await;
        println!("{:?}", model);
    }
}
