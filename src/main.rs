mod attack;
mod stats;

use clap::Parser;
use attack::{run_attack, AttackConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 目标IP地址
    #[arg(short, long)]
    target: String,
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
    #[arg(short, long, default_value = "GET")]
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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = AttackConfig {
        target: args.target,
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
