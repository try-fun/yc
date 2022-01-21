use std::error::Error;
use std::result::Result;
use yc::tester;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let mut v = Vec::new();
    // let u1 = 0u128;
    // let u2 = 0u128;
    // v.push(u1 - u2);
    // println!("{}", u1 - u2);
    tester::run(10, 1, "http://localhost:8000/h").await;
    Ok(())
}
