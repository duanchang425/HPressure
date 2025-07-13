use crate::{attacks::{AttackConfig, AttackResult}, stats::StatsArc, utils};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub async fn run_icmp_attack(config: AttackConfig, stats: StatsArc) -> AttackResult {
    println!("🌊 ICMP洪水攻击启动");
    println!("目标: {}", config.target);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("数据包大小: {} 字节", config.packet_size);
    println!("伪装源IP: {}", if config.spoof_source { "启用" } else { "禁用" });
    println!("随机数据包大小: {}", if config.random_packet_size { "启用" } else { "禁用" });
    println!();

    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);
    
    // 调整参数
    let adjusted_connections = utils::adjust_connections_by_mode(config.connections, &config.mode, "icmp");
    let (min_delay, max_delay) = utils::get_delay_by_mode(&config.mode, "icmp");

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", adjusted_connections);
    println!("延迟范围: {}ms - {}ms", min_delay, max_delay);
    println!();

    // 创建攻击任务
    let mut tasks = Vec::new();
    for _ in 0..adjusted_connections {
        let target = config.target.clone();
        let packet_size = config.packet_size;
        let random_packet_size = config.random_packet_size;
        let min_packet_size = config.min_packet_size;
        let max_packet_size = config.max_packet_size;
        let stats = stats.clone();
        let duration = duration.clone();

        let task = tokio::spawn(async move {
            let start = Instant::now();
            
            while start.elapsed() < duration {
                let request_start = Instant::now();

                // 确定数据包大小
                let actual_packet_size = if random_packet_size {
                    rand::thread_rng().gen_range(min_packet_size..=max_packet_size)
                } else {
                    packet_size
                };

                // 生成ICMP数据包（简化实现）
                let packet = utils::generate_random_packet(actual_packet_size);
                
                // 模拟发送ICMP包（实际实现需要原始socket）
                {
                    let mut stats = stats.lock().unwrap();
                    stats.add_request(true, actual_packet_size as u64, 0);
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

            println!("⏱️ [{}s] ICMP RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
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
            println!("⏳ ICMP攻击进度: {}/{} 秒", elapsed, config.duration);
        }
        println!("✅ ICMP攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("✅ ICMP攻击完成！");
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