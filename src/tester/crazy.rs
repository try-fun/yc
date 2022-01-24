use crate::libs::tools::pretty;
use reqwest::{Client, Url};
use std::time;
use tokio::sync::mpsc;
use tokio::task;

//  疯狂模式
pub async fn run(t: u64, url: &str) {
    let now = time::Instant::now();
    let client = Client::new();
    let (tx, rx) = mpsc::channel(100);
    let u = url.to_string();
    let t1 = task::spawn(async move {
        format(t, u, rx).await;
    });

    for i in 0.. {
        if now.elapsed().as_secs() >= t {
            println!("发送总数: {}", i);
            break;
        }

        let tx_x = tx.clone();
        let url = Url::parse(url).unwrap();
        let response = client.get(url).send();
        tokio::spawn(async move {
            let start = time::Instant::now();
            match response.await {
                Ok(resp) => {
                    let body = resp.bytes().await;
                    match body {
                        Ok(x) => {
                            let stop = time::Instant::now();
                            if let Err(e) =
                                tx_x.send((200, (stop - start).as_millis(), x.len())).await
                            {
                                panic!("{}", e);
                            }
                        }
                        Err(_e) => {
                            if let Err(e) = tx_x.send((500, 0u128, 0)).await {
                                panic!("{}", e);
                            }
                        }
                    }
                }
                Err(_e) => {
                    if let Err(e) = tx_x.send((400, 0u128, 0)).await {
                        panic!("{}", e);
                    }
                }
            }
        });
    }
    drop(tx);

    if let Ok(_) = t1.await {}
}

pub async fn format(t: u64, url: String, mut rx: mpsc::Receiver<(u32, u128, usize)>) {
    //请求信息
    println!("开始处理: {}", url);
    println!("压测时长: {} 秒", t);
    let start = time::Instant::now(); //开始计时
    let mut ok_count = 0i64; //成功数
    let mut failed_count = 0i64; //失败数
    let mut missed_count = 0i64; //丢失数
    let mut data_len = 0f64; //数据长度 bytes
    let mut times: Vec<u128> = Vec::with_capacity(10);

    while let Some(x) = rx.recv().await {
        if x.0 == 200 {
            ok_count += 1;
            times.push(x.1); //毫秒
        } else if x.0 == 500 {
            failed_count += 1;
            times.push(x.1); //毫秒
        } else {
            missed_count += 1;
        }
        data_len += x.2 as f64; //数据 长度
    }

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
    println!("   丢失: {}", missed_count);
    println!("下载数据: {}", pretty::bytes(data_len as f64));
    println!("数据/秒: {}", pretty::bytes(avg_size));

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
