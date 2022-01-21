pub mod libs;

pub mod tester {

    use futures::{stream, StreamExt};
    use reqwest::Client;
    use std::sync::Arc;
    use std::sync::Mutex;
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
        let t1 = task::spawn(async move {
            format(n, rx).await;
        });

        // 创建任务
        let client = Client::new();
        let reqs = vec![url; n];
        let tasks = stream::iter(reqs)
            .map(|url| {
                let client = &client;
                async move {
                    let resp = client.get(url).send().await?;
                    let status = resp.status();
                    resp.bytes()
                        .await
                        .map(|byte| (time::Instant::now(), status, byte))
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
                        (start, status, x) => {
                            if let Err(e) = tx
                                .send((200, start, stop, x.len(), status.to_string()))
                                .await
                            {
                                panic!("{}", e)
                            }
                        }
                    },
                    _ => {
                        if let Err(e) = tx
                            .send((
                                500,
                                stop,
                                stop,
                                0,
                                reqwest::StatusCode::SERVICE_UNAVAILABLE.to_string(),
                            ))
                            .await
                        {
                            panic!("{}", e);
                        }
                    }
                }
            })
            .await;
        drop(tx);

        // 等待任务结束
        if let Ok(_) = t1.await {}
    }

    pub async fn format(
        n: usize,
        mut rx: mpsc::Receiver<(u32, time::Instant, time::Instant, usize, String)>,
    ) {
        let vec: Vec<u128> = Vec::with_capacity(10);
        let tuple = Arc::new(Mutex::new((0, 0, 0, 0, 0, 0, 0, String::from(""), vec)));
        {
            let tuple1 = tuple.clone();
            let now = time::Instant::now(); //计时
            task::spawn(async move {
                print_header();
                loop {
                    std::thread::sleep(time::Duration::from_secs(1));
                    let t = match tuple1.lock() {
                        Ok(x) => x,
                        Err(e) => panic!("{}", e),
                    };

                    // 减去sleep的1秒
                    let secs = now.elapsed().as_secs();
                    let reqc = t.0 + t.1; //响应数量

                    // QPS QPS = req/sec = 请求数/秒
                    let qps = reqc as f64 / if secs <= 0 { 1 } else { secs } as f64;
                    let bytes_per = t.6 as f64 / if secs <= 0 { 1 } else { secs } as f64;

                    // 最长耗时
                    let max = match t.8.iter().max() {
                        Some(x) => x,
                        _ => &0u128,
                    };
                    // 最短耗时
                    let min = match t.8.iter().min() {
                        Some(x) => x,
                        _ => &0u128,
                    };
                    // 平均耗时
                    let sum: u128 = t.8.iter().sum();
                    let avg = sum / if t.8.len() == 0 { 1 } else { t.8.len() } as u128;

                    println!(
                        "{0:>4}s│{1:>7}│{2:>7}│{3:>7.1}│{4:>8}│{5:>8}│{6:>8}│{7:>10}│{8:>10.2}│{9:<8}",
                        secs, t.0, t.1, qps, max, min, avg, t.6, bytes_per, t.7,
                    );

                    if n == reqc as usize {
                        break;
                    }
                }
            });
        }

        while let Some(x) = rx.recv().await {
            let mut t = tuple.try_lock().unwrap();
            //TODO:poisoned lock: another task failed inside
            t.8.push(x.1.elapsed().as_micros() - x.2.elapsed().as_micros());
            if x.0 == 200 {
                t.0 += 1; //成功数
            } else {
                t.1 += 1; //失败数
            }

            // id, start, stop, 200, x.len(), status.to_string()
            t.6 += x.3 as u64; //body长度
            t.7 = x.4; //状态码
        }
    }

    fn print_header() {
        println!();
        // 打印的时长都为毫秒 总请数
        println!(
            "─────┬───────┬───────┬───────┬────────┬────────┬────────┬──────────┬──────────┬────────"
        );
        println!(
            " 耗时│ 成功数│ 失败数│  qps  │最长耗时│最短耗时│平均耗时│ 下载字节 │ 字节每秒 │ 状态码"
        );
        println!(
            "─────┼───────┼───────┼───────┼────────┼────────┼────────┼──────────┼──────────┼────────"
        );
        return;
    }
}
