pub mod libs;

pub mod tester {

    use crate::libs::tools::pretty;
    use futures::{stream, StreamExt};
    use indicatif::ProgressBar;
    use reqwest::{Client, Url};
    use std::time;
    use tokio::sync::mpsc;
    use tokio::task;
    /**
    运行压力测试

    c: 并发数量
    n: 请求次数

    # Examples

    */
    pub async fn run(n: usize, c: usize, url: &str) {
        // 初始化channle
        let (tx, rx) = mpsc::channel(1);

        // 统计输出
        task::spawn(async move {
            format(n, c, rx).await;
        });

        // 创建任务
        let client = Client::new();
        let reqs = vec![url; n];
        let tasks = stream::iter(reqs)
            .map(|url| {
                let client = &client;
                async move {
                    let url = Url::parse(url).unwrap();
                    let resp = client.get(url).send().await?;
                    resp.bytes().await.map(|byte| (time::Instant::now(), byte))
                }
            })
            .buffer_unordered(c);

        // 处理结果
        tasks
            .for_each(|x| async {
                let tx = tx.clone();
                let stop = time::Instant::now();
                match x {
                    Ok(x) => match x {
                        (start, x) => {
                            if let Err(e) = tx.send((200, (stop - start).as_nanos(), x.len())).await
                            {
                                panic!("{}", e)
                            }
                        }
                    },
                    _ => {
                        if let Err(e) = tx.send((500, 0u128, 0)).await {
                            panic!("{}", e);
                        }
                    }
                }
            })
            .await;
        drop(tx);
    }

    pub async fn format(n: usize, c: usize, mut rx: mpsc::Receiver<(u32, u128, usize)>) {
        let mut ok_count = 0; //成功数
        let mut failed_count = 0; //失败数
        let mut data_len: usize = 0; //数据长度 bytes
        let mut times: Vec<u128> = Vec::with_capacity(10);
        let start = time::Instant::now(); //开始计时

        let pb = ProgressBar::new(n as u64); // 进度条

        while let Some(x) = rx.recv().await {
            if x.0 == 200 {
                ok_count += 1;
            } else {
                failed_count += 1;
            }

            data_len += x.2; //数据 长度
            times.push(x.1); //微秒
            pb.inc(1); //显示进度
        }
        pb.finish_with_message("finished!");

        // 输出统计
        let stop = time::Instant::now();
        let sent_total = ok_count - failed_count;
        let avg_time = (times.iter().sum::<u128>() / if n == 0 { 1 } else { n } as u128) as f32;
        let avg_size = (data_len / if sent_total == 0 { 1 } else { sent_total }) as f64;
        println!("并发数: {}", c);
        println!("请求次数: {}", n);
        if (stop - start).as_secs() > 0 {
            println!("耗时: {} s", (stop - start).as_secs());
        } else {
            println!("耗时: {} ms", (stop - start).as_millis());
        };

        println!("请求数/秒: {} 次", n);
        println!("成功数: {}", ok_count);
        println!("失败数: {}", failed_count);
        println!("丢失数: {}", n - sent_total);
        println!("下载数据: {}", pretty::bytes(data_len as f64));
        println!("数据/秒: {}", pretty::bytes(avg_size));

        println!(
            "最长时间: {} ns",
            match times.iter().max() {
                Some(x) => x,
                _ => &0u128,
            }
        );
        println!(
            "最短时间: {} ns",
            match times.iter().min() {
                Some(x) => x,
                _ => &0u128,
            }
        );
        println!("平均时间: {} ns", avg_time);
    }
}
