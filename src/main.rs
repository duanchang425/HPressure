/*
 * HPressure - 高性能DDoS压力测试工具
 * Copyright (C) 2024 HPressure Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use clap::Parser;
use HPressure::{attacks::{AttackConfig, AttackType, AttackManager}, AppConfig};
use HPressure::interactive::start_interactive_mode;

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
    /// 攻击类型 (http/udp/tcp/icmp/slowloris/syn)
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
    /// 伪造源IP (SYN)
    #[arg(long)]
    spoof_ip: bool,
    /// 交互模式
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // 加载配置文件
    let app_config = AppConfig::load();
    println!("📋 已加载配置文件");

    // 交互模式
    if args.interactive {
        start_interactive_mode().await;
        return;
    }

    // 命令行模式
    if let Some(target) = args.target {
        let attack_manager = AttackManager::new();
        
        // 使用命令行参数或配置文件默认值
        let connections = args.connections.unwrap_or_else(|| {
            match args.attack_type.to_lowercase().as_str() {
                "http" | "https" => app_config.default_http_connections,
                "udp" => app_config.default_udp_connections,
                "tcp" => app_config.default_tcp_connections,
                "icmp" => app_config.default_icmp_connections,
                "slowloris" => app_config.default_slowloris_connections,
                "syn" => app_config.default_syn_connections,
                _ => app_config.default_http_connections,
            }
        });
        
        let duration = args.duration.unwrap_or(app_config.default_duration);
        let mode = args.mode.unwrap_or(app_config.default_mode);
        let packet_size = args.packet_size.unwrap_or(app_config.default_packet_size);

        // 根据攻击类型创建配置
        let attack_type = AttackType::from_str(&args.attack_type).unwrap_or(AttackType::Http);
        let mut config = AttackConfig::new(attack_type.clone(), target, args.port);
        
        // 设置通用参数
        config.connections = connections;
        config.duration = duration;
        config.mode = mode;
        config.packet_size = packet_size;

        // 根据攻击类型设置特定参数
        match attack_type {
            AttackType::Http => {
                config.https = args.https;
                config.method = args.method.to_uppercase();
                config.post_data = args.post_data;
                config.user_agent = args.user_agent;
            }
            AttackType::Tcp => {
                config.payload_type = args.payload_type;
                config.custom_payload = args.custom_payload;
            }
            AttackType::Icmp => {
                config.spoof_source = args.spoof_source;
                config.random_packet_size = args.random_packet_size;
                config.min_packet_size = args.min_packet_size;
                config.max_packet_size = args.max_packet_size;
            }
            AttackType::Slowloris => {
                config.timeout = args.timeout;
                config.keep_alive = args.keep_alive;
                config.random_headers = args.random_headers;
                config.min_interval = args.min_interval;
                config.max_interval = args.max_interval;
            }
            AttackType::Syn => {
                config.spoof_ip = args.spoof_ip;
            }
            _ => {}
        }

        // 执行攻击
        println!("🚀 开始 {} 攻击...", attack_type.as_str());
        let result = attack_manager.run_attack(config).await;
        
        // 显示结果
        display_result(&result);
    } else {
        println!("❌ 请指定目标IP地址");
        println!("使用方法: cargo run -- --target example.com --attack-type http");
    }
}

fn display_result(result: &HPressure::attacks::AttackResult) {
    println!("\n📊 攻击结果:");
    println!("总请求数: {}", result.total_requests);
    println!("成功请求: {}", result.successful_requests);
    println!("失败请求: {}", result.failed_requests);
    println!("发送字节: {}", result.bytes_sent);
    println!("接收字节: {}", result.bytes_received);
    println!("平均RPS: {:.2}", result.average_rps);
    println!("成功率: {:.2}%", result.success_rate);
    println!("持续时间: {:.2}秒", result.duration.as_secs_f64());
}
