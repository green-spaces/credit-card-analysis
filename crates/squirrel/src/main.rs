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

    println!("Welcome to Squirrel");
    let mut input = String::new();
    // Application Loop
    while io::stdin().read_line(&mut input).is_ok() {
        let trimmed_input = input.trim_end();
        println!("Input: {}", trimmed_input);

        if trimmed_input == "q" {
            break;
        }
        input.clear();
    }

    Ok(())
}
