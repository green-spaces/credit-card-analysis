use credit_card_app::{
    bill_models::{BillLine, BillLineString},
    utils,
};

fn main() {
    let sample = "./bills/July-2022.csv";

    let csv_contents = utils::read_file_to_string(sample);
    let bill_lines = BillLineString::parse_csv(csv_contents)
        .into_iter()
        .filter_map(|i| BillLine::try_from(i).ok())
        .collect::<Vec<BillLine>>();

    for line in bill_lines {
        println!("{:?}", line);
    }
}
