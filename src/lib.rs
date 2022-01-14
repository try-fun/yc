pub mod libs;

pub mod client {
    use std::sync::Arc;
    use tokio::sync::mpsc;

    use crate::libs::args::Args;
    /**
    运行压力测试

    c: 并发数量
    n: 请求次数

    # Examples
    ```rust
    #[tokio::test]
    async fn test(){
        use yc::client;
        let (c, n) = (2, 10);
        client::run(c, n).await;
    }
    ```
    */
    pub async fn run(args: Args) {
        // 1.初始化channle
        let (tx, mut rx) = mpsc::channel(10);

        let url = Arc::new(args.url);
        // 2.创建任务
        for _ in 0..args.c {
            let tx_n = tx.clone();
            let url = Arc::clone(&url);
            tokio::spawn(async move {
                for _ in 0..args.n {
                    let resp = reqwest::get(url.as_str()).await;
                    if let Err(e) = tx_n.send(resp).await {
                        println!("receiver dropped {}", &e.to_string());
                    }
                }
            });
        }

        // drop(tx) 确保任务返回None结果,如果没有drop(tx),则不会返回None
        drop(tx);

        // 3.处理结果
        while let Some(res) = rx.recv().await {
            if let Ok(x) = res {
                println!("got = {:?}", x.status());
            }
        }
    }

    pub fn format() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sque() {
        assert_eq!(2, 5 % 3);
    }

    #[tokio::test]
    async fn for_run() {
        client::run(libs::args::Args {
            n: 1,
            c: 10,
            url: "http://localhost:8000/h".to_string(),
        })
        .await;
        assert_eq!("", "")
    }

    #[test]
    fn format() {
        client::format();
    }
}
