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
            .enumerate()
            .map(|(i, url)| {
                let client = &client;
                async move {
                    let resp = client.get(url).send().await?;
                    let status = resp.status();
                    resp.bytes()
                        .await
                        .map(|byte| (i, time::Instant::now(), status, byte))
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
                        (id, start, status, x) => {
                            if let Err(e) = tx
                                .send((id, start, stop, 200, x.len(), status.to_string()))
                                .await
                            {
                                println!("{}", e)
                            }
                        }
                    },
                    _ => {
                        if let Err(_) = tx
                            .send((
                                0,
                                stop,
                                stop,
                                500,
                                0,
                                reqwest::StatusCode::SERVICE_UNAVAILABLE.to_string(),
                            ))
                            .await
                        {
                            println!("message sent failed")
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
        mut rx: mpsc::Receiver<(usize, time::Instant, time::Instant, u32, usize, String)>,
    ) {
        let tuple = Arc::new(Mutex::new((0, 0, 0, 0, 0, 0, 0, 0, "", "")));
        {
            let tuple1 = tuple.clone();
            let now = time::Instant::now();
            task::spawn(async move {
                print_header();
                loop {
                    std::thread::sleep(time::Duration::from_secs(1));
                    let t = tuple1.lock().unwrap();

                    let secs = now.elapsed().as_secs();
                    let reqc = t.0 + t.1;
                    // QPS QPS = req/sec = 请求数/秒
                    let qps = reqc / if secs <= 0 { 1 } else { secs };

                    println!(
                        "{0:<5}│{1:<7}│{2:<7}│{3:<7}│{4:<8}│{5:<8}│{6:<8}│{7:<8}│{8:<8}│{9:<8}",
                        secs, t.0, t.1, qps, t.3, t.4, t.5, t.6, t.7, t.8,
                    );

                    if n == reqc as usize {
                        break;
                    }
                }
            });
        }

        while let Some(x) = rx.recv().await {
            let mut t = tuple.lock().unwrap();
            if x.3 == 200 {
                t.0 += 1;
            } else {
                t.1 += 1;
            }
        }
    }

    fn print_header() {
        println!();
        // 打印的时长都为毫秒 总请数
        println!(
            "─────┬───────┬───────┬───────┬────────┬────────┬────────┬────────┬────────┬────────"
        );
        println!(
            " 耗时│ 成功数│ 失败数│  qps  │最长耗时│最短耗时│平均耗时│下载字节│字节每秒│ 状态码"
        );
        println!(
            "─────┼───────┼───────┼───────┼────────┼────────┼────────┼────────┼────────┼────────"
        );
        return;
    }
}
