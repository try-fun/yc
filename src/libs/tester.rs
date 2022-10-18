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
pub async fn run(
    n: usize,
    c: usize,
    url: &str,
    timeout: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url).unwrap();
    // 初始化channle
    let (tx, rx) = mpsc::channel(10);

    // 统计输出
    let u = url.to_string();
    let t1 = task::spawn(async move {
        format(n, c, u, rx).await;
    });

    // 创建任务
    let reqs = vec![url; n];
    let tasks = stream::iter(reqs)
        .map(|url| async move {
            let client = Client::builder()
                .timeout(time::Duration::from_secs(timeout))
                .build()
                .unwrap();
            let start = time::Instant::now();
            let resp = client.get(url).send().await?;
            resp.bytes().await.map(|byte| (start, byte))
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
                        if let Err(e) = tx.send((200, (stop - start).as_millis(), x.len())).await {
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

    if let Ok(_) = t1.await {}
    Ok(())
}

pub async fn format(n: usize, c: usize, url: String, mut rx: mpsc::Receiver<(u32, u128, usize)>) {
    //请求信息
    println!("开始处理: {}", url);
    println!("并发: {} 个", c);
    println!("请求: {} 次", n);

    let start = time::Instant::now(); //开始计时
    let mut ok_count = 0; //成功数
    let mut failed_count = 0; //失败数
    let mut data_len = 0f64; //数据长度 bytes
    let mut times: Vec<u128> = Vec::with_capacity(n);
    let pb = ProgressBar::new(n as u64); // 进度条

    while let Some(x) = rx.recv().await {
        if x.0 == 200 {
            ok_count += 1;
        } else {
            failed_count += 1;
        }

        data_len += x.2 as f64; //数据 长度
        times.push(x.1); //微秒
        pb.inc(1); //显示进度
    }
    pb.finish_with_message("finished!");

    // 输出统计
    let stop = time::Instant::now();
    let min = (stop - start).as_micros() as f64 / 1000000f64;
    let sent_total = (ok_count + failed_count) as f64;
    let avg_time = (times.iter().sum::<u128>()
        / if sent_total == 0f64 { 1f64 } else { sent_total } as u128) as f32;
    let avg_size = (data_len / if sent_total == 0f64 { 1f64 } else { sent_total }) as f64;

    println!();
    if min > 0f64 {
        println!(
            "   耗时: {:.2} s",
            ((stop - start).as_micros() as f64 / 1000000f64)
        );
    } else {
        println!("   耗时: {} ms", (stop - start).as_millis());
    };
    println!(
        "请求/秒: {:.2} 次/秒",
        sent_total as f64 / if min == 0f64 { 1f64 } else { min }
    );

    println!("   成功: {}", ok_count);
    println!("   失败: {}", failed_count);
    println!("   丢失: {}", n as f64 - sent_total);
    println!("下载数据: {}", pretty::bytes(data_len as u64));
    println!("数据/秒: {}", pretty::bytes(avg_size as u64));

    println!(
        "最长耗时: {} ms",
        match times.iter().max() {
            Some(x) => x,
            _ => &0u128,
        }
    );
    println!(
        "最短耗时: {} ms",
        match times.iter().min() {
            Some(x) => x,
            _ => &0u128,
        }
    );
    println!("平均耗时: {} ms", avg_time);
}
