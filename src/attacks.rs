use crate::stats::StatsArc;
use std::time::Duration;

/// 攻击类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AttackType {
    Http,
    Udp,
    Tcp,
    Icmp,
    Slowloris,
    Syn,
}

impl AttackType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "http" | "https" => Some(AttackType::Http),
            "udp" => Some(AttackType::Udp),
            "tcp" => Some(AttackType::Tcp),
            "icmp" => Some(AttackType::Icmp),
            "slowloris" => Some(AttackType::Slowloris),
            "syn" => Some(AttackType::Syn),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AttackType::Http => "http",
            AttackType::Udp => "udp",
            AttackType::Tcp => "tcp",
            AttackType::Icmp => "icmp",
            AttackType::Slowloris => "slowloris",
            AttackType::Syn => "syn",
        }
    }
}

/// 统一的攻击配置
#[derive(Debug, Clone)]
pub struct AttackConfig {
    pub attack_type: AttackType,
    pub target: String,
    pub port: u16,
    pub connections: usize,
    pub duration: u64,
    pub mode: String,
    
    // HTTP特定配置
    pub https: bool,
    pub method: String,
    pub post_data: Option<String>,
    pub user_agent: Option<String>,
    
    // UDP/TCP/ICMP特定配置
    pub packet_size: usize,
    pub payload_type: String,
    pub custom_payload: Option<String>,
    
    // ICMP特定配置
    pub spoof_source: bool,
    pub random_packet_size: bool,
    pub min_packet_size: usize,
    pub max_packet_size: usize,
    
    // Slowloris特定配置
    pub timeout: u64,
    pub keep_alive: bool,
    pub random_headers: bool,
    pub min_interval: u64,
    pub max_interval: u64,
    
    // SYN特定配置
    pub spoof_ip: bool,
}

impl AttackConfig {
    pub fn new(attack_type: AttackType, target: String, port: u16) -> Self {
        Self {
            attack_type,
            target,
            port,
            connections: 1000,
            duration: 60,
            mode: "normal".to_string(),
            https: false,
            method: "GET".to_string(),
            post_data: None,
            user_agent: None,
            packet_size: 1024,
            payload_type: "random".to_string(),
            custom_payload: None,
            spoof_source: false,
            random_packet_size: false,
            min_packet_size: 64,
            max_packet_size: 1024,
            timeout: 30,
            keep_alive: false,
            random_headers: false,
            min_interval: 10,
            max_interval: 50,
            spoof_ip: false,
        }
    }

    pub fn with_connections(mut self, connections: usize) -> Self {
        self.connections = connections;
        self
    }

    pub fn with_duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_mode(mut self, mode: String) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_packet_size(mut self, packet_size: usize) -> Self {
        self.packet_size = packet_size;
        self
    }
}

/// 攻击结果
#[derive(Debug)]
pub struct AttackResult {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub duration: Duration,
    pub average_rps: f64,
    pub success_rate: f64,
}

impl AttackResult {
    pub fn new(stats: &crate::stats::Stats, duration: Duration) -> Self {
        let total_requests = stats.total_requests;
        let successful_requests = stats.successful_requests;
        let failed_requests = stats.failed_requests;
        let bytes_sent = stats.bytes_sent;
        let bytes_received = stats.bytes_received;
        
        let average_rps = if duration.as_secs() > 0 {
            total_requests as f64 / duration.as_secs() as f64
        } else {
            0.0
        };
        
        let success_rate = if total_requests > 0 {
            (successful_requests as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_requests,
            successful_requests,
            failed_requests,
            bytes_sent,
            bytes_received,
            duration,
            average_rps,
            success_rate,
        }
    }
}

/// 攻击执行器特征
pub trait AttackExecutor {
    fn execute(&self, config: &AttackConfig) -> AttackResult;
}

/// 攻击管理器
pub struct AttackManager {
    stats: StatsArc,
}

impl AttackManager {
    pub fn new() -> Self {
        Self {
            stats: StatsArc::default(),
        }
    }

    pub async fn run_attack(&self, config: AttackConfig) -> AttackResult {
        let start_time = std::time::Instant::now();
        
        // 根据攻击类型选择执行器
        match config.attack_type {
            AttackType::Http => self.run_http_attack(config).await,
            AttackType::Udp => self.run_udp_attack(config).await,
            AttackType::Tcp => self.run_tcp_attack(config).await,
            AttackType::Icmp => self.run_icmp_attack(config).await,
            AttackType::Slowloris => self.run_slowloris_attack(config).await,
            AttackType::Syn => self.run_syn_attack(config).await,
        }
    }

    async fn run_http_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::http::run_http_attack(config, self.stats.clone()).await
    }

    async fn run_udp_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::udp::run_udp_attack(config, self.stats.clone()).await
    }

    async fn run_tcp_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::tcp::run_tcp_attack(config, self.stats.clone()).await
    }

    async fn run_icmp_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::icmp::run_icmp_attack(config, self.stats.clone()).await
    }

    async fn run_slowloris_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::slowloris::run_slowloris_attack(config, self.stats.clone()).await
    }

    async fn run_syn_attack(&self, config: AttackConfig) -> AttackResult {
        crate::attacks_impl::syn::run_syn_attack(config, self.stats.clone()).await
    }
} 