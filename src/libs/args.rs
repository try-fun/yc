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
                "-c" => {
                    i += 1;
                    self.c = args[i].parse::<u32>().unwrap();
                }
                "-n" => {
                    i += 1;
                    self.n = args[i].parse::<u32>().unwrap();
                }
                _ => {
                    self.url = args[i].to_string();
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
