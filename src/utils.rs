use rand::Rng;
use std::time::Duration;

/// 生成随机User-Agent
pub fn get_random_user_agent() -> &'static str {
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
    user_agents[rand::thread_rng().gen_range(0..user_agents.len())]
}

/// 生成随机语言设置
pub fn get_random_language() -> &'static str {
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
    languages[rand::thread_rng().gen_range(0..languages.len())]
}

/// 生成随机头部值
pub fn generate_random_header_value() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let length = rng.gen_range(5..15);
    (0..length).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

/// 生成随机数据包
pub fn generate_random_packet(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<u8>()).collect()
}

/// 根据模式调整延迟
pub fn get_delay_by_mode(mode: &str, attack_type: &str) -> (u64, u64) {
    match (mode.to_lowercase().as_str(), attack_type) {
        ("stealth", "syn") => (1, 5),
        ("normal", "syn") => (0, 1),
        ("aggressive", "syn") => (0, 0),
        ("stealth", "udp") => (50, 200),
        ("normal", "udp") => (10, 50),
        ("aggressive", "udp") => (1, 10),
        ("stealth", "tcp") => (20, 100),
        ("normal", "tcp") => (5, 25),
        ("aggressive", "tcp") => (1, 5),
        ("stealth", "icmp") => (50, 200),
        ("normal", "icmp") => (10, 50),
        ("aggressive", "icmp") => (1, 10),
        ("stealth", "slowloris") => (50, 200),
        ("normal", "slowloris") => (10, 50),
        ("aggressive", "slowloris") => (1, 10),
        _ => (10, 50),
    }
}

/// 根据模式调整并发数
pub fn adjust_connections_by_mode(base_connections: usize, mode: &str, attack_type: &str) -> usize {
    let multiplier = match (mode.to_lowercase().as_str(), attack_type) {
        ("stealth", "syn") => 1.5,
        ("normal", "syn") => 4.0,
        ("aggressive", "syn") => 8.0,
        ("stealth", "udp") => 0.3,
        ("normal", "udp") => 1.0,
        ("aggressive", "udp") => 3.0,
        ("stealth", "tcp") => 0.4,
        ("normal", "tcp") => 1.0,
        ("aggressive", "tcp") => 4.0,
        ("stealth", "icmp") => 0.5,
        ("normal", "icmp") => 1.0,
        ("aggressive", "icmp") => 3.0,
        ("stealth", "slowloris") => 0.5,
        ("normal", "slowloris") => 1.0,
        ("aggressive", "slowloris") => 2.0,
        _ => 1.0,
    };
    (base_connections as f64 * multiplier) as usize
}

/// 格式化字节大小
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// 安全睡眠，避免阻塞
pub async fn safe_sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
} 