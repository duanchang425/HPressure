use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_http_connections: usize,
    pub default_udp_connections: usize,
    pub default_duration: u64,
    pub default_packet_size: usize,
    pub default_mode: String,
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_http_connections: 1000,
            default_udp_connections: 1000,
            default_duration: 60,
            default_packet_size: 1024,
            default_mode: "normal".to_string(),
            max_connections: 10000,
            timeout_seconds: 30,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = "config.json";
        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => config,
                        Err(e) => {
                            eprintln!("配置文件解析错误: {}", e);
                            Self::default()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("读取配置文件失败: {}", e);
                    Self::default()
                }
            }
        } else {
            let config = Self::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let config_path = "config.json";
        match serde_json::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(config_path, content) {
                    eprintln!("保存配置文件失败: {}", e);
                }
            }
            Err(e) => {
                eprintln!("序列化配置文件失败: {}", e);
            }
        }
    }
} 