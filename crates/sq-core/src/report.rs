use super::aggregate::CategoryAggregate;
use crate::line_item::Money;
use std::fmt::Debug;

pub fn descending_report<U: Money>(agg: &CategoryAggregate<U>)
where
    <U as TryInto<f64>>::Error: Debug,
{
    let mut cates = agg.category_spend.iter().collect::<Vec<_>>();
    cates[..].sort_by(|a, b| b.1.cmp(&a.1));

    let longest_word = cates.iter().map(|row| row.0.len()).max().unwrap();
    let total_spend = cates.iter().map(|row| *row.1).sum::<U>();

    let header = format!(
        "\n{:<longest_word$} |\t{:^7}\t| {}",
        "Category", "Debt", "Percentage"
    );
    let header_length = header.len();
    println!("{header}");
    println!("{:-<header_length$}", "-");

    for (name, &debt) in cates {
        let percent_spend: f64 =
            (debt.try_into().unwrap() / total_spend.try_into().unwrap()) * 100.0;
        // let percent_spend = 0;
        println!("{name:<longest_word$} |\t{debt:>7.2}\t|   {percent_spend:<.1} %");
    }

    println!("{:-<header_length$}", "-");
    println!("Total Spend | {total_spend:.2}");
}
