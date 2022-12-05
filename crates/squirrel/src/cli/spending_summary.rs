//! Produces a spending summary

use chrono::NaiveDate;
use clap::Args;

use crate::Squirrel;

use sq_core::{
    aggregate::{AggregateFrom, CategoryAggregate},
    line_item::{LineItem, LineItemReduction},
    report,
};

#[derive(Debug, Args)]
pub struct SpendingSummaryCommand {
    #[clap(short, long)]
    month: Option<u8>,
}

impl SpendingSummaryCommand {
    pub async fn display_summary(&self, squirrel: &Squirrel) {
        let bl = squirrel.all_dc_and_bl().await;
        let line_items = bl
            .iter()
            .map(|(cat, bls)| {
                bls.iter()
                    .map(|bl| {
                        // TODO missing returns
                        let flow = bl.debit.unwrap_or_default().round() as i32;
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

        let reduction = LineItemReduction::reduce(line_items, Vec::new());
        let cat_agg = CategoryAggregate::aggregate_from(reduction);

        report::descending_report(&cat_agg);
    }
}
