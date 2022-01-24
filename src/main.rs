use std::error::Error;
use std::result::Result;
use yc::libs::args::Args;
use yc::tester::{comm, crazy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new().parse();
    let url: &str = &args.url.clone()[..];
    if url.trim() == "" {
        println!("need url");
        return Ok(());
    }

    if args.t > 0 {
        crazy::run(args.t as u64, url).await;
    } else {
        comm::run(args.n as usize, args.c as usize, url).await;
    }

    Ok(())
}
