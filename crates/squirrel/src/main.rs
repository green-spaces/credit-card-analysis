use std::io;

use squirrel::{Error, Squirrel};

const TEST_DATABASE: &str = "sqlite://test.db";
const PROD_DATABASE: &str = "sqlite://prod.db";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let sq = Squirrel::new(PROD_DATABASE);

    // TODO Move into a command after starting squirrel
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(file_path) = args.get(1) {
        return sq.load_csv(file_path).await;
    }

    println!("Squirrel - Nut Tracker");
    let mut input = String::new();
    // Application Loop
    println!("Actions: (c)atigorize, (s)pending summery, (dc) display categories,  (q)uit");
    while io::stdin().read_line(&mut input).is_ok() {
        let trimmed_input = input.trim_end();

        match trimmed_input {
            "c" => {
                squirrel::ui_actions::categorize_descriptions::categorize_descriptions(&sq).await
            }
            "s" => squirrel::ui_actions::spending_summary::spending_summary(&sq).await,
            "dc" => {
                squirrel::ui_actions::display_categories::display_categories(&sq).await;
            }
            "q" => break,
            _ => {
                input = String::new()
            },
        }

        input.clear();
    }

    Ok(())
}
