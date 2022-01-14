use std::error::Error;
use std::result::Result;
use yc::client;
use yc::libs::args::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new().parse();
    client::run(args).await;
    Ok(())
}
