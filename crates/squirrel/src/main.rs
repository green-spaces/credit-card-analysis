use std::io;

use squirrel::{Error, Squirrel};

const TEST_DATABASE: &str = "sqlite://test.db";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let sq = Squirrel::new(TEST_DATABASE);

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
            "s" => println!("Not implemented yet"),
            "dc" => {
                squirrel::ui_actions::display_categories::display_categories(&sq).await;
            }
            "q" => break,
            _ => todo!(),
        }

        input.clear();
    }

    Ok(())
}
