use credit_card_app::{
    bill_models::{BillLineString, ParsedBillLine},
    utils,
};

use credit_card_app::db;

#[tokio::main]
async fn main() {
    let db = db::Database::new("sqlite://test.db");

    let sample = "./bills/2022-04.csv";

    let csv_contents = utils::read_file_to_string(sample);
    let csv_model = db.csv_create(csv_contents.clone()).await;
    println!("{:#?}", csv_model);

    let bill_lines = BillLineString::parse_csv(csv_contents)
        .into_iter()
        .filter_map(|i| ParsedBillLine::try_from(i).ok())
        .collect::<Vec<ParsedBillLine>>();

    for line in bill_lines {
        println!("{:?}", line);
        let model = db.bill_line_create(line, csv_model.id).await;
        println!("{:?}", model);
    }

    for l in db.bld_read_all().await {
        println!("{:?}", l);
    }
}
