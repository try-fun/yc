use reqwest::Url;
use std::env;
pub struct Args {
    // 请求次数
    pub n: u32,
    // 并发数量
    pub c: u32,
    // url
    pub url: String,
}

impl Args {
    pub fn new() -> Self {
        Self {
            c: 1,
            n: 30,
            url: String::from(""),
        }
    }
    pub fn parse(&mut self) -> Self {
        let args: Vec<String> = env::args().collect();
        for mut i in 0..args.len() {
            let x = args[i].as_str();
            match x {
                "-c" | "-c=" => {
                    i += 1;
                    self.c = args[i].parse::<u32>().unwrap();
                }
                "-n" | "-n=" => {
                    i += 1;
                    self.n = args[i].parse::<u32>().unwrap();
                }
                _ => {
                    let url = match Url::parse(&args[i]) {
                        Ok(x) => x.to_string(),
                        Err(_) => "".to_string(),
                    };
                    self.url = url;
                }
            };
        }
        Self {
            c: self.c,
            n: self.n,
            url: self.url.clone(),
        }
    }
}
