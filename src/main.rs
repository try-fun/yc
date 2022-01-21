use std::error::Error;
use std::result::Result;
use yc::libs::args::Args;
use yc::tester;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new().parse();
    let url: &str = &args.url.clone()[..];
    tester::run(args.n as usize, args.c as usize, url).await;
    Ok(())
}
