//! Produces a spending summary

use clap::Args;

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
    pub fn summarize(&self, database_url: &str) -> SpendingSummary {
        todo!()
    }
}
