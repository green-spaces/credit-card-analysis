//! Produces a spending summary

use clap::Args;

use crate::Squirrel;

#[derive(Debug, Args)]
pub struct SpendingSummaryCommand {
    #[clap(short, long)]
    month: Option<u8>,
}

#[derive(Debug)]
pub struct SpendingSummary {
    // list of expenses and how much was spend
    expenses: Vec<(String, f64)>,
}

impl SpendingSummaryCommand {
    pub async fn summarize(&self, squirrel: &Squirrel) -> SpendingSummary {
        let bl = squirrel.all_dc_and_bl().await;
        println!("{bl:#?}");
        todo!()
    }
}
