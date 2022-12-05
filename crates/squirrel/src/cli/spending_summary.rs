//! Produces a spending summary

use crate::Squirrel;
use chrono::{Datelike, NaiveDate};
use clap::Args;
use num_traits::cast::FromPrimitive;

use sq_core::{
    aggregate::{AggregateFrom, CategoryAggregate},
    line_item::{LineFilter, LineItem, LineItemReduction},
    report,
};

#[derive(Debug, Args)]
pub struct SpendingSummaryCommand {
    #[clap(short, long)]
    month: Option<u32>,
}

impl SpendingSummaryCommand {
    pub async fn display_summary(&self, squirrel: &Squirrel) {
        let bl = squirrel.all_dc_and_bl().await;
        let line_items = bl
            .iter()
            .map(|(cat, bls)| {
                bls.iter()
                    .map(|bl| {
                        // Credits are refunds and should be negative
                        let flow = bl
                            .debit
                            .or(bl.credit.map(|i| -i))
                            .unwrap_or_default()
                            .round() as i32;
                        let date =
                            NaiveDate::parse_from_str(&bl.transaction_data, "%Y-%m-%d").unwrap();
                        LineItem::new(flow, &cat.name, date)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let mut filters = vec![LineFilter::new(Box::new(|ln| ln.category != "Payment"))];
        if let Some(month) = self.month {
            let now = chrono::Utc::now().naive_local().date();
            let year = now.year();
            let start = NaiveDate::from_ymd(year, month, 1);
            let end = start.checked_add_months(chrono::Months::new(1)).unwrap();

            filters.push(LineFilter::item_date_on_or_after(start));
            filters.push(LineFilter::item_date_before(end));
            println!(
                "Month: {} - {}",
                chrono::Month::from_u32(month).unwrap().name(),
                year
            );
        };
        let reduction = LineItemReduction::reduce(line_items, filters);
        let cat_agg = CategoryAggregate::aggregate_from(reduction);

        report::descending_report(&cat_agg);
    }
}
