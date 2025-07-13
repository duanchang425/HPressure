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

// 核心模块
pub mod config;
pub mod stats;
pub mod utils;

// 攻击模块
pub mod attacks;
pub mod attacks_impl;

// 交互模块
pub mod interactive;

// 重新导出主要类型
pub use config::AppConfig;
pub use stats::{Stats, StatsArc};
pub use attacks::{AttackType, AttackConfig, AttackResult, AttackManager}; 