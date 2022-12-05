mod spending_summary;

use clap::{Parser, Subcommand};

use spending_summary::SpendingSummaryCommand;

use crate::Squirrel;

const PROD_DATABASE: &str = "sqlite://prod.db";

#[derive(Subcommand, Debug)]
enum Actions {
    /// Summary of spending on the
    #[command(short_flag = 'S')]
    SpendingSummary(SpendingSummaryCommand),
    /// Add a Expense/Bill
    Expense,
    /// Update description of an expense
    ExpenseDescription,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long)]
    database_url: Option<String>,

    #[command(subcommand)]
    command: Actions,
}

impl Cli {
    /// Parses the
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub async fn execute(&self) {
        let db_url = self
            .database_url
            .clone()
            .unwrap_or_else(|| PROD_DATABASE.to_string());

        let sq = Squirrel::new(&db_url);

        match &self.command {
            Actions::SpendingSummary(command) => {
                let spending_summary = command.display_summary(&sq).await;
                // display summary
                println!("{spending_summary:#?}");
            }
            _ => todo!("Unimplemented actions"),
        }

        todo!()
    }
}
