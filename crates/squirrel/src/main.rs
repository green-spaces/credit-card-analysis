
use squirrel::{Cli, Error, Squirrel};

#[allow(dead_code)]
const TEST_DATABASE: &str = "sqlite://test.db";
const PROD_DATABASE: &str = "sqlite://prod.db";

/*
Actions:
- Categorize Descriptions
- Add bill
- Display spending summary
    - filter by month optional
*/

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    let _sq = Squirrel::new(PROD_DATABASE);

    let cli = Cli::parse();

    println!("{cli:?}");

    cli.execute().await;

    Ok(())
}
