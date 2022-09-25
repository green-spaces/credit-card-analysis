use crate::Squirrel;

pub async fn spending_summary(sq: &Squirrel) {
    let res = sq.all_dc_and_bl().await;
    println!("Spending summary");
    let table = table_builder(res);
    display_table(table);
}

fn display_table(mut table: Vec<(String, f64)>) {
    let longest_word = table.iter().map(|row| row.0.len()).max().unwrap();
    let total_spend = table.iter().map(|row| row.1).sum::<f64>();

    table.sort_by_key(|row| -(row.1 * 100.0) as i64);

    let header = format!(
        "\n{:<longest_word$} |\t{:^7}\t| {}",
        "Category", "Debt", "Percentage"
    );
    let header_length = header.len();
    println!("{header}");
    println!("{:-<header_length$}", "-");

    for (name, debt) in table {
        let percent_spend = (debt / total_spend) * 100.0;
        println!("{name:<longest_word$} |\t{debt:>7.2}\t| {percent_spend:<.1} %");
    }

    println!("{:-<header_length$}", "-");
    println!("Total Spend | {total_spend:.2}");
}

fn table_builder(
    data: Vec<(
        db_entity::entity::description_category::Model,
        Vec<db_entity::entity::bill_line::Model>,
    )>,
) -> Vec<(String, f64)> {
    let raw_table = data
        .iter()
        .map(|(cat, bls)| (cat.name.to_string(), total_debit(bls)))
        .collect::<Vec<_>>();

    raw_table
}

fn total_debit(bill_lines: &[db_entity::entity::bill_line::Model]) -> f64 {
    bill_lines.iter().filter_map(|bl| bl.debit).sum::<f64>()
}
