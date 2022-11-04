mod spending_summary;

use clap::{Parser, Subcommand};

use spending_summary::SpendingSummaryCommand;

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

    pub fn execute(&self) {
        let db_url = self.database_url.clone().unwrap_or_default();
        match &self.command {
            Actions::SpendingSummary(command) => {
                let spending_summary = command.summarize(&db_url);
                // display summary
                println!("{spending_summary:#?}");
            }
            _ => todo!("Unimplemented actions"),
        }

        todo!()
    }
}
