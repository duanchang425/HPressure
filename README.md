# 高性能DDoS工具

这是一个使用Rust编写的高性能DDoS压力测试工具，专为教育目的和合法的压力测试而设计。

## ⚠️ 重要声明

**此工具仅用于：**
- 教育目的
- 对自己拥有的系统进行压力测试
- 获得明确授权的安全测试

**严禁用于：**
- 攻击他人的系统
- 未经授权的网络攻击
- 任何非法活动

使用者需承担所有法律责任。

## 功能特性

- 🚀 **极高性能**: 支持HTTP/UDP/TCP/ICMP四大攻击类型，异步高并发
- 📊 **实时监控**: 实时显示RPS（每秒请求数）和成功率
- 🔧 **灵活配置**: 支持自定义目标、端口、并发数、持续时间、数据包大小
- 📈 **详细统计**: 提供详细的攻击统计信息
- 🛡️ **连接优化**: 自动优化连接参数以获得最佳性能
- 🔄 **多种HTTP方法**: 支持GET和POST请求
- 🎭 **智能伪装**: 随机User-Agent、Referer、X-Forwarded-For等真实浏览器风格头部
- 🎯 **攻击模式**: 支持normal、stealth、aggressive三种模式
- 🌊 **UDP洪水攻击**: 支持UDP数据包洪水攻击
- 🌪️ **TCP洪水攻击**: 支持TCP洪水攻击，支持random/http/custom三种payload
- 🎯 **ICMP洪水攻击**: 支持ICMP洪水攻击，支持伪装源IP和随机数据包大小
- 💬 **交互模式**: 友好的交互式用户界面

## 安装和编译

### 前置要求
- Rust 1.70+ 
- Cargo

### 编译步骤

```bash
# 克隆项目
git clone <repository-url>
cd DDoS

# 编译项目
cargo build --release

# 运行（示例）
cargo run --release -- --target 127.0.0.1 --port 8080 --connections 1000 --duration 60
```

## 使用方法

### 基本用法

```bash
# 对本地服务器进行压力测试
cargo run --release -- --target 127.0.0.1 --port 8080

# 指定并发连接数
cargo run --release -- --target 127.0.0.1 --port 8080 --connections 2000

# 指定持续时间（秒）
cargo run --release -- --target 127.0.0.1 --port 8080 --duration 120

# 使用HTTPS
cargo run --release -- --target example.com --port 443 --https
```

### 命令行参数

| 参数 | 简写 | 默认值 | 描述 |
|------|------|--------|------|
| `--target` | `-t` | 必需 | 目标IP地址或域名 |
| `--port` | `-p` | 80 | 目标端口 |
| `--connections` | `-c` | 1000 | 并发连接数 |
| `--duration` | `-d` | 60 | 攻击持续时间（秒） |
| `--https` | | false | 使用HTTPS协议 |
| `--method` | | GET | HTTP方法 (GET/POST) |
| `--post-data` | | | POST请求的数据 |
| `--user-agent` | | | 自定义User-Agent |
| `--mode` | `-m` | normal | 攻击模式 (normal/stealth/aggressive) |
| `--attack-type` | `-a` | http | 攻击类型 (http/udp/tcp/icmp) |
| `--packet-size` | | 1024 | UDP/TCP/ICMP数据包大小 |
| `--payload-type` | | random | TCP负载类型 (random/http/custom) |
| `--custom-payload` | | | TCP自定义负载内容 |
| `--spoof-source` | | false | ICMP伪装源IP |
| `--random-packet-size` | | false | ICMP随机数据包大小 |
| `--min-packet-size` | | 64 | ICMP最小数据包大小 |
| `--max-packet-size` | | 1024 | ICMP最大数据包大小 |
| `--interactive` | `-i` | false | 启动交互模式 |

### 使用示例

```bash
# 对本地Web服务器进行1分钟的压力测试
cargo run --release -- --target 127.0.0.1 --port 8080 --connections 500 --duration 60

# 对远程服务器进行高强度测试
cargo run --release -- --target example.com --port 80 --connections 2000 --duration 300

# 对HTTPS服务进行测试
cargo run --release -- --target api.example.com --port 443 --https --connections 1000

# 发送POST请求
cargo run --release -- --target example.com --port 80 --method POST --post-data "username=test&password=123456"

# 隐蔽模式攻击
cargo run --release -- --target example.com --port 80 --mode stealth --connections 500

# 激进模式攻击
cargo run --release -- --target example.com --port 80 --mode aggressive --connections 2000

# 自定义User-Agent
cargo run --release -- --target example.com --port 80 --user-agent "MyBot/1.0"

# UDP洪水攻击
cargo run --release -- --target example.com --port 80 --attack-type udp --packet-size 2048

# TCP洪水攻击（随机负载）
cargo run --release -- --target example.com --port 80 --attack-type tcp --packet-size 2048 --payload-type random

# TCP洪水攻击（HTTP负载）
cargo run --release -- --target example.com --port 80 --attack-type tcp --packet-size 2048 --payload-type http

# TCP洪水攻击（自定义负载）
cargo run --release -- --target example.com --port 80 --attack-type tcp --packet-size 2048 --payload-type custom --custom-payload "helloDDOS"

# ICMP洪水攻击（基本）
cargo run --release -- --target example.com --attack-type icmp --packet-size 1024

# ICMP洪水攻击（伪装源IP）
cargo run --release -- --target example.com --attack-type icmp --spoof-source --packet-size 1024

# ICMP洪水攻击（随机数据包大小）
cargo run --release -- --target example.com --attack-type icmp --random-packet-size --min-packet-size 64 --max-packet-size 2048

# ICMP洪水攻击（隐蔽模式）
cargo run --release -- --target example.com --attack-type icmp --mode stealth --packet-size 512

# 交互模式
cargo run --release -- --interactive
```

## 配置文件支持

工具支持通过 `config.json` 配置文件自定义默认参数。

- 首次运行时会自动在程序目录下生成 `config.json`。
- 你可以手动编辑该文件，修改默认并发数、持续时间、数据包大小、模式等。
- 命令行参数或交互输入留空时，将自动采用配置文件中的默认值。

### 配置文件示例
```json
{
  "default_http_connections": 1000,
  "default_udp_connections": 1000,
  "default_tcp_connections": 1000,
  "default_icmp_connections": 1000,
  "default_duration": 60,
  "default_packet_size": 1024,
  "default_mode": "normal",
  "max_connections": 10000,
  "timeout_seconds": 30
}
```

| 字段 | 说明 |
|------|------|
| default_http_connections | HTTP/HTTPS 默认并发连接数 |
| default_udp_connections  | UDP 默认并发连接数 |
| default_tcp_connections  | TCP 默认并发连接数 |
| default_icmp_connections | ICMP 默认并发连接数 |
| default_duration         | 默认攻击持续时间（秒） |
| default_packet_size      | UDP/TCP/ICMP 默认数据包大小（字节） |
| default_mode             | 默认攻击模式（normal/stealth/aggressive） |
| max_connections          | 最大允许连接数 |
| timeout_seconds          | 网络超时时间（秒） |

> **提示**：如需批量测试或统一调整默认参数，建议直接编辑 `config.json`。

## 性能优化建议

### 系统级优化

1. **增加文件描述符限制**
   ```bash
   # Linux
   ulimit -n 65536
   
   # macOS
   sudo launchctl limit maxfiles 65536 200000
   ```

2. **调整网络参数**
   ```bash
   # Linux
   echo 'net.core.somaxconn = 65535' >> /etc/sysctl.conf
   echo 'net.ipv4.tcp_max_syn_backlog = 65535' >> /etc/sysctl.conf
   sysctl -p
   ```

3. **使用高性能网络**
   - 使用有线网络连接
   - 确保网络带宽充足

### 工具级优化

1. **调整并发数**: 根据目标服务器性能调整并发连接数
2. **监控资源**: 观察CPU和内存使用情况
3. **网络延迟**: 考虑网络延迟对性能的影响

## 输出说明

### 实时监控
```
📊 RPS: 1250 | 成功: 1245 | 失败: 5 | 总计: 1250
```
- **RPS**: 每秒请求数
- **成功**: 成功响应的请求数
- **失败**: 失败的请求数
- **总计**: 总请求数

### 最终统计
```
📊 最终攻击统计:
总运行时间: 60.00秒
目标: 127.0.0.1:8080
并发连接数: 1000
总请求数: 75000
成功请求: 74800
失败请求: 200
平均RPS: 1250
发送字节: 15 MB
接收字节: 45 MB
成功率: 99.7%
```

## 攻击模式

### 三种攻击模式

1. **Normal模式** (默认)
   - 并发数：用户指定值
   - 延迟：50-150ms随机延迟
   - 特点：平衡性能和隐蔽性

2. **Stealth模式** (隐蔽)
   - 并发数：用户指定值的50%
   - 延迟：100-500ms随机延迟
   - 特点：添加Referer、Sec-Fetch等伪装头、X-Forwarded-For等
   - 适用：需要高度隐蔽的场景

3. **Aggressive模式** (激进)
   - 并发数：用户指定值的200%及以上
   - 延迟：10-50ms随机延迟
   - 特点：最大化攻击强度
   - 适用：对性能要求极高的场景

### UDP攻击模式

1. **Normal模式** (默认)
   - 并发数：用户指定值
   - 延迟：10-50ms随机延迟
   - 特点：平衡性能和隐蔽性

2. **Stealth模式** (隐蔽)
   - 并发数：用户指定值的30%
   - 延迟：50-200ms随机延迟
   - 特点：高度隐蔽，减少被检测风险
   - 适用：需要高度隐蔽的场景

3. **Aggressive模式** (激进)
   - 并发数：用户指定值的300%
   - 延迟：1-10ms随机延迟
   - 特点：最大化UDP攻击强度
   - 适用：对性能要求极高的场景

### ICMP攻击模式

1. **Normal模式** (默认)
   - 并发数：用户指定值
   - 延迟：10-50ms随机延迟
   - 特点：平衡性能和隐蔽性

2. **Stealth模式** (隐蔽)
   - 并发数：用户指定值的50%
   - 延迟：50-200ms随机延迟
   - 特点：高度隐蔽，减少被检测风险
   - 适用：需要高度隐蔽的场景

3. **Aggressive模式** (激进)
   - 并发数：用户指定值的300%
   - 延迟：1-10ms随机延迟
   - 特点：最大化ICMP攻击强度
   - 适用：对性能要求极高的场景

### TCP攻击模式

1. **Normal模式** (默认)
   - 并发数：用户指定值
   - 延迟：5-25ms随机延迟
   - 特点：平衡性能和隐蔽性

2. **Stealth模式** (隐蔽)
   - 并发数：用户指定值的40%
   - 延迟：20-100ms随机延迟
   - 特点：高度隐蔽，减少被检测风险
   - 适用：需要高度隐蔽的场景

3. **Aggressive模式** (激进)
   - 并发数：用户指定值的400%
   - 延迟：1-5ms随机延迟
   - 特点：最大化TCP攻击强度
   - 适用：对性能要求极高的场景

## 技术架构

### 核心组件

1. **异步HTTP客户端**: 使用`reqwest`库，支持连接池和超时控制
2. **并发任务管理**: 使用`tokio`运行时，支持数千个并发任务
3. **统计收集**: 使用线程安全的统计收集器
4. **进度监控**: 实时显示攻击进度和性能指标
5. **智能伪装**: 随机User-Agent、Referer、X-Forwarded-For等真实浏览器风格头部

### 性能特性

- **连接复用**: 自动复用HTTP连接，减少连接建立开销
- **异步I/O**: 非阻塞I/O操作，最大化CPU利用率
- **内存优化**: 最小化内存分配，减少GC压力
- **网络优化**: 自动调整网络参数以获得最佳性能

## 安全注意事项

1. **仅测试自己的系统**: 确保只对你有权限的系统进行测试
2. **监控系统资源**: 避免对测试系统造成永久性损害
3. **遵守法律法规**: 确保所有测试活动都符合当地法律
4. **记录测试活动**: 保留测试记录以备后续分析

## 故障排除

### 常见问题

1. **连接被拒绝**
   - 检查目标服务器是否运行
   - 确认端口号正确
   - 检查防火墙设置

2. **性能不佳**
   - 增加并发连接数
   - 检查网络带宽
   - 优化系统参数

3. **内存使用过高**
   - 减少并发连接数
   - 检查是否有内存泄漏
   - 重启工具

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug cargo run --release -- --target 127.0.0.1 --port 8080
```

## 许可证

本项目仅供教育和研究目的使用。使用者需承担所有法律责任。

## 贡献

欢迎提交Issue和Pull Request来改进这个工具。

## 免责声明

本工具仅用于合法的安全测试和教育目的。使用者必须确保遵守所有适用的法律法规，并承担使用本工具的所有责任。 