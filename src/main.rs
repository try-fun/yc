use std::error::Error;
use std::result::Result;
use yc::tester;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tester::run(100, 10, "http://localhost:8000/h").await;
    Ok(())
}
