use crate::{AttackConfig, UdpFloodConfig, TcpFloodConfig};
use std::io::{self, Write};

pub async fn start_interactive_mode() {
    println!("🎯 高性能DDoS工具 - 交互模式");
    println!("==================================");
    println!();

    loop {
        println!("请选择攻击类型:");
        println!("1. HTTP/HTTPS 攻击");
        println!("2. UDP 洪水攻击");
        println!("3. TCP 洪水攻击");
        println!("4. 退出");
        println!();

        print!("请输入选择 (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                if let Some(config) = get_http_config() {
                    crate::attack::run_attack(config).await;
                }
            }
            "2" => {
                if let Some(config) = get_udp_config() {
                    crate::udp_flood::run_udp_flood(config).await;
                }
            }
            "3" => {
                if let Some(config) = get_tcp_config() {
                    crate::tcp_flood::run_tcp_flood(config).await;
                }
            }
            "4" => {
                println!("👋 再见！");
                break;
            }
            _ => {
                println!("❌ 无效选择，请重新输入");
            }
        }

        println!();
        println!("按回车键继续...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn get_http_config() -> Option<AttackConfig> {
    println!("\n🌐 HTTP/HTTPS 攻击配置");
    println!("========================");

    // 目标
    print!("目标IP/域名: ");
    io::stdout().flush().unwrap();
    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    if target.is_empty() {
        println!("❌ 目标不能为空");
        return None;
    }

    // 端口
    print!("端口 (默认80): ");
    io::stdout().flush().unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim().parse::<u16>().unwrap_or(80);

    // 并发数
    print!("并发连接数 (默认1000): ");
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(1000);

    // 持续时间
    print!("持续时间(秒) (默认60): ");
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(60);

    // HTTPS
    print!("使用HTTPS? (y/N): ");
    io::stdout().flush().unwrap();
    let mut https = String::new();
    io::stdin().read_line(&mut https).unwrap();
    let https = https.trim().to_lowercase() == "y";

    // HTTP方法
    print!("HTTP方法 (GET/POST, 默认GET): ");
    io::stdout().flush().unwrap();
    let mut method = String::new();
    io::stdin().read_line(&mut method).unwrap();
    let method = method.trim().to_uppercase();
    let method = if method.is_empty() || (method != "GET" && method != "POST") {
        "GET".to_string()
    } else {
        method
    };

    // POST数据
    let post_data = if method == "POST" {
        print!("POST数据: ");
        io::stdout().flush().unwrap();
        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();
        let data = data.trim().to_string();
        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    } else {
        None
    };

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认normal): ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        "normal".to_string()
    } else {
        mode
    };

    // 自定义User-Agent
    print!("自定义User-Agent (留空使用随机): ");
    io::stdout().flush().unwrap();
    let mut user_agent = String::new();
    io::stdin().read_line(&mut user_agent).unwrap();
    let user_agent = user_agent.trim().to_string();
    let user_agent = if user_agent.is_empty() {
        None
    } else {
        Some(user_agent)
    };

    Some(AttackConfig {
        target,
        port,
        connections,
        duration,
        https,
        method,
        post_data,
        user_agent,
        mode,
    })
}

fn get_udp_config() -> Option<UdpFloodConfig> {
    println!("\n🌊 UDP洪水攻击配置");
    println!("===================");

    // 目标
    print!("目标IP/域名: ");
    io::stdout().flush().unwrap();
    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    if target.is_empty() {
        println!("❌ 目标不能为空");
        return None;
    }

    // 端口
    print!("端口 (默认80): ");
    io::stdout().flush().unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim().parse::<u16>().unwrap_or(80);

    // 并发数
    print!("并发连接数 (默认1000): ");
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(1000);

    // 持续时间
    print!("持续时间(秒) (默认60): ");
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(60);

    // 数据包大小
    print!("数据包大小(字节) (默认1024): ");
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(1024);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认normal): ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        "normal".to_string()
    } else {
        mode
    };

    Some(UdpFloodConfig {
        target,
        port,
        connections,
        duration,
        packet_size,
        mode,
    })
}

fn get_tcp_config() -> Option<TcpFloodConfig> {
    println!("\n🌊 TCP洪水攻击配置");
    println!("===================");

    // 目标
    print!("目标IP/域名: ");
    io::stdout().flush().unwrap();
    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    if target.is_empty() {
        println!("❌ 目标不能为空");
        return None;
    }

    // 端口
    print!("端口 (默认80): ");
    io::stdout().flush().unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim().parse::<u16>().unwrap_or(80);

    // 并发数
    print!("并发连接数 (默认1000): ");
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(1000);

    // 持续时间
    print!("持续时间(秒) (默认60): ");
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(60);

    // 数据包大小
    print!("数据包大小(字节) (默认1024): ");
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(1024);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认normal): ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        "normal".to_string()
    } else {
        mode
    };

    // 负载类型
    print!("负载类型 (random/http/custom, 默认random): ");
    io::stdout().flush().unwrap();
    let mut payload_type = String::new();
    io::stdin().read_line(&mut payload_type).unwrap();
    let payload_type = payload_type.trim().to_lowercase();
    let payload_type = if payload_type.is_empty() || (payload_type != "random" && payload_type != "http" && payload_type != "custom") {
        "random".to_string()
    } else {
        payload_type
    };

    // 自定义负载
    let custom_payload = if payload_type == "custom" {
        print!("自定义负载数据: ");
        io::stdout().flush().unwrap();
        let mut payload = String::new();
        io::stdin().read_line(&mut payload).unwrap();
        let payload = payload.trim().to_string();
        if payload.is_empty() {
            None
        } else {
            Some(payload)
        }
    } else {
        None
    };

    Some(TcpFloodConfig {
        target,
        port,
        connections,
        duration,
        packet_size,
        mode,
        payload_type,
        custom_payload,
    })
} 