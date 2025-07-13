# HPressure - 高性能DDoS压力测试工具

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

本项目是用 Rust 编写的高性能 DDoS 压力测试工具，支持多种攻击类型，适用于合法授权下的安全测试与教育研究。

**许可证**: 本项目采用 [GNU General Public License v3.0](LICENSE.txt) 许可证。

## ⚠️ 免责声明
- 本工具仅限于教育、研究和对授权目标的压力测试。
- 禁止用于任何非法用途，使用者需自行承担法律责任。

## 功能特性
- 支持 HTTP、UDP、TCP、ICMP、Slowloris、SYN 六大攻击类型
- 统一的攻击接口，代码结构清晰
- 支持多种攻击模式（normal/stealth/aggressive）
- 支持自定义并发、持续时间、数据包大小、HTTP方法、User-Agent 等
- 实时统计与进度输出
- 配置文件支持，参数灵活
- 模块化设计，易于扩展和维护
- 交互模式和命令行模式

## 安装与编译

### 环境要求
- Rust 1.70+
- Cargo

### 编译
```bash
cargo build --release
```

## 使用方法

### 基本用法
```bash
# HTTP攻击
cargo run --release -- --target example.com --port 80 --attack-type http --connections 1000 --duration 60

# UDP洪水
cargo run --release -- --target example.com --port 80 --attack-type udp --packet-size 2048

# TCP洪水
cargo run --release -- --target example.com --port 80 --attack-type tcp --packet-size 2048 --payload-type random

# ICMP洪水
cargo run --release -- --target example.com --attack-type icmp --packet-size 1024

# SYN洪水
cargo run --release -- --target example.com --port 80 --attack-type syn --packet-size 1024

# Slowloris攻击
cargo run --release -- --target example.com --port 80 --attack-type slowloris
```

### 命令行参数
| 参数 | 说明 | 默认值 |
|------|------|--------|
| --target | 目标IP或域名 | 必需 |
| --port | 目标端口 | 80 |
| --connections | 并发连接数 | 1000 |
| --duration | 持续时间（秒） | 60 |
| --attack-type | 攻击类型（http/udp/tcp/icmp/slowloris/syn） | http |
| --mode | 攻击模式（normal/stealth/aggressive） | normal |
| --packet-size | 数据包大小 | 1024 |
| --payload-type | TCP负载类型（random/http/custom） | random |
| --custom-payload | TCP自定义负载 |  |
| --https | 是否使用HTTPS | false |
| --method | HTTP方法 | GET |
| --post-data | POST数据 |  |
| --user-agent | 自定义User-Agent |  |
| --spoof-source | ICMP伪造源IP | false |
| --random-packet-size | ICMP随机包大小 | false |
| --min-packet-size | ICMP最小包大小 | 64 |
| --max-packet-size | ICMP最大包大小 | 1024 |
| --timeout | Slowloris超时 | 30 |
| --keep-alive | Slowloris保持连接 | false |
| --random-headers | Slowloris随机头部 | false |
| --min-interval | Slowloris最小间隔(ms) | 10 |
| --max-interval | Slowloris最大间隔(ms) | 50 |
| --spoof-ip | SYN伪造源IP | false |

### 配置文件
首次运行会自动生成 `config.json`，可手动编辑：
```json
{
  "default_http_connections": 1000,
  "default_udp_connections": 1000,
  "default_tcp_connections": 1000,
  "default_icmp_connections": 1000,
  "default_slowloris_connections": 500,
  "default_syn_connections": 1000,
  "default_duration": 60,
  "default_packet_size": 1024,
  "default_mode": "normal",
  "max_connections": 10000,
  "timeout_seconds": 30
}
```

## 代码结构
```
HPressure/
  src/
    lib.rs           # 主模块，统一导出
    main.rs          # 二进制入口点
    utils.rs         # 工具函数
    stats.rs         # 统计模块
    config.rs        # 配置模块
    interactive.rs   # 交互模式
    attacks.rs       # 攻击类型与统一接口
    attacks_impl/    # 各攻击类型实现
      mod.rs         # 模块导出
      http.rs        # HTTP攻击实现
      udp.rs         # UDP攻击实现
      tcp.rs         # TCP攻击实现
      icmp.rs        # ICMP攻击实现
      syn.rs         # SYN攻击实现
      slowloris.rs   # Slowloris攻击实现
```

## 输出示例
```
⏱️ [2s] SYN RPS: 100 | 成功: 200 | 失败: 0 | 总计: 200
⏳ 攻击进度: 2/10 秒
✅ SYN攻击完成！
📊 最终统计: ...
```

## 许可证
本项目采用 GNU General Public License v3.0 (GPL-3.0) 许可证。

GPL-3.0 是一个自由软件许可证，确保软件保持自由开放。使用本软件时，您必须：
- 保留原始版权声明
- 在修改版本中明确标注修改内容
- 将修改后的代码同样以GPL-3.0许可证发布
- 提供源代码访问方式

完整许可证文本请查看 [LICENSE.txt](LICENSE.txt) 文件。

## 贡献
欢迎提交PR和Issue！

**注意**: 根据GPL-3.0许可证要求，所有贡献的代码都将以相同的GPL-3.0许可证发布。 