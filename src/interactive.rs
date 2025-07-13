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

use crate::attacks::{AttackConfig, AttackType, AttackManager, AttackResult};
use crate::config::AppConfig;
use std::io::{self, Write};

pub async fn start_interactive_mode() {
    println!("🎯 高性能DDoS工具 - 交互模式");
    println!("==================================");
    
    // 加载配置文件
    let app_config = AppConfig::load();
    println!("📋 已加载配置文件");
    println!();

    let attack_manager = AttackManager::new();

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
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
                }
            }
            "2" => {
                if let Some(config) = get_udp_config(&app_config) {
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
                }
            }
            "3" => {
                if let Some(config) = get_tcp_config(&app_config) {
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
                }
            }
            "4" => {
                if let Some(config) = get_icmp_config(&app_config) {
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
                }
            }
            "5" => {
                if let Some(config) = get_slowloris_config(&app_config) {
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
                }
            }
            "6" => {
                if let Some(config) = get_syn_config(&app_config) {
                    let result = attack_manager.run_attack(config).await;
                    display_result(&result);
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

fn display_result(result: &AttackResult) {
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

    let mut config = AttackConfig::new(AttackType::Http, target, port);
    config.connections = connections;
    config.duration = duration;
    config.https = https;
    config.method = method;
    config.post_data = post_data;
    config.user_agent = user_agent;
    config.mode = mode;

    Some(config)
}

fn get_udp_config(app_config: &AppConfig) -> Option<AttackConfig> {
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
    print!("数据包大小 (默认{}): ", app_config.default_packet_size);
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

    let mut config = AttackConfig::new(AttackType::Udp, target, port);
    config.connections = connections;
    config.duration = duration;
    config.packet_size = packet_size;
    config.mode = mode;

    Some(config)
}

fn get_tcp_config(app_config: &AppConfig) -> Option<AttackConfig> {
    println!("\n🌐 TCP洪水攻击配置");
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
    print!("数据包大小 (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

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
        print!("自定义负载: ");
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

    let mut config = AttackConfig::new(AttackType::Tcp, target, port);
    config.connections = connections;
    config.duration = duration;
    config.packet_size = packet_size;
    config.payload_type = payload_type;
    config.custom_payload = custom_payload;
    config.mode = mode;

    Some(config)
}

fn get_icmp_config(app_config: &AppConfig) -> Option<AttackConfig> {
    println!("\n📡 ICMP洪水攻击配置");
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
    print!("数据包大小 (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 伪造源IP
    print!("伪造源IP? (y/N): ");
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
    print!("最小数据包大小 (默认64): ");
    io::stdout().flush().unwrap();
    let mut min_packet_size = String::new();
    io::stdin().read_line(&mut min_packet_size).unwrap();
    let min_packet_size = min_packet_size.trim().parse::<usize>().unwrap_or(64);

    // 最大数据包大小
    print!("最大数据包大小 (默认1024): ");
    io::stdout().flush().unwrap();
    let mut max_packet_size = String::new();
    io::stdin().read_line(&mut max_packet_size).unwrap();
    let max_packet_size = max_packet_size.trim().parse::<usize>().unwrap_or(1024);

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

    let mut config = AttackConfig::new(AttackType::Icmp, target, 0); // ICMP不需要端口
    config.connections = connections;
    config.duration = duration;
    config.packet_size = packet_size;
    config.spoof_source = spoof_source;
    config.random_packet_size = random_packet_size;
    config.min_packet_size = min_packet_size;
    config.max_packet_size = max_packet_size;
    config.mode = mode;

    Some(config)
}

fn get_slowloris_config(app_config: &AppConfig) -> Option<AttackConfig> {
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
    print!("最小间隔(ms) (默认10): ");
    io::stdout().flush().unwrap();
    let mut min_interval = String::new();
    io::stdin().read_line(&mut min_interval).unwrap();
    let min_interval = min_interval.trim().parse::<u64>().unwrap_or(10);

    // 最大间隔
    print!("最大间隔(ms) (默认50): ");
    io::stdout().flush().unwrap();
    let mut max_interval = String::new();
    io::stdin().read_line(&mut max_interval).unwrap();
    let max_interval = max_interval.trim().parse::<u64>().unwrap_or(50);

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

    let mut config = AttackConfig::new(AttackType::Slowloris, target, port);
    config.connections = connections;
    config.duration = duration;
    config.timeout = timeout;
    config.keep_alive = keep_alive;
    config.random_headers = random_headers;
    config.min_interval = min_interval;
    config.max_interval = max_interval;
    config.mode = mode;

    Some(config)
}

fn get_syn_config(app_config: &AppConfig) -> Option<AttackConfig> {
    println!("\n🔗 SYN洪水攻击配置");
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
    print!("数据包大小 (默认{}): ", app_config.default_packet_size);
    io::stdout().flush().unwrap();
    let mut packet_size = String::new();
    io::stdin().read_line(&mut packet_size).unwrap();
    let packet_size = packet_size.trim().parse::<usize>().unwrap_or(app_config.default_packet_size);

    // 伪造源IP
    print!("伪造源IP? (y/N): ");
    io::stdout().flush().unwrap();
    let mut spoof_ip = String::new();
    io::stdin().read_line(&mut spoof_ip).unwrap();
    let spoof_ip = spoof_ip.trim().to_lowercase() == "y";

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

    let mut config = AttackConfig::new(AttackType::Syn, target, port);
    config.connections = connections;
    config.duration = duration;
    config.packet_size = packet_size;
    config.spoof_ip = spoof_ip;
    config.mode = mode;

    Some(config)
} 