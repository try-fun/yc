use clap;
use clap::Parser;
use yc::libs::tester;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Number of requests
    #[clap(short = 'n', long, value_parser, default_value_t = 100)]
    pub requests: u32,

    /// Number of multiple requests to make at a time
    #[clap(short = 'c', long, value_parser, default_value_t = 1)]
    pub concurrency: u32,

    /// Wait for each response Default is 30 seconds
    #[clap(short = 't', long, value_parser, default_value_t = 30)]
    pub timeout: u64,

    /// URL
    #[clap(short = 'u', long, value_parser)]
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let mut url = args.url;
    if !url.contains("http") {
        url = format!("http://{}", url);
    }

    let t = tester::run(
        args.requests as usize,
        args.concurrency as usize,
        &url,
        args.timeout,
    )
    .await;

    match t {
        Err(e) => {
            println!("{}", e.to_string());
        }
        _ => {}
    }

    Ok(())
}
