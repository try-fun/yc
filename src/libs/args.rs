use reqwest::Url;
use std::env;
pub struct Args {
    // 时间秒
    pub t: u32,
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
            t: 0,
            c: 1,  //默认
            n: 10, //默认
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
                    self.c = match args[i].parse::<u32>() {
                        Ok(x) => x,
                        Err(_) => panic!("-c need number"),
                    }
                }
                "-n" | "-n=" => {
                    i += 1;
                    self.n = match args[i].parse::<u32>() {
                        Ok(x) => x,
                        Err(_) => panic!("-n need number"),
                    }
                }
                "-t" | "-t=" => {
                    i += 1;
                    self.t = match args[i].parse::<u32>() {
                        Ok(x) => x,
                        Err(_) => panic!("-t need number"),
                    }
                }
                _ => {
                    // 解析url
                    if self.url == "" {
                        self.url = if let Ok(x) = Url::parse(&args[i]) {
                            let mut surl = String::from("");

                            // 自动补充http头,如果缺失http://
                            if x.as_str().contains("http") {
                                surl.push_str(x.as_str())
                            } else {
                                surl.push_str("http://");
                                surl.push_str(x.as_str());
                            }

                            surl
                        } else {
                            "".to_string()
                        };
                    }
                }
            };
        }
        Self {
            t: self.t,
            c: self.c,
            n: self.n,
            url: self.url.clone(),
        }
    }
}
