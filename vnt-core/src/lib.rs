pub(crate) mod compression;
pub mod context;
pub mod core;
pub mod crypto;
pub(crate) mod ethernet;
pub(crate) mod event_script;
pub(crate) mod fec;
pub mod log_manager;
pub mod managed_config;
pub mod nat;
pub mod network_info;
pub mod protocol;
#[cfg(not(any(target_os = "android", target_os = "ios", target_os = "tvos")))]
mod system_subnet_routes;
pub mod tls;
pub(crate) mod tun;
pub mod tunnel_core;
pub mod utils;

pub mod api;
pub(crate) mod enhanced_tunnel;
pub mod port_mapping;
pub(crate) mod runtime_config;

#[cfg(feature = "internal-bench")]
#[doc(hidden)]
pub mod internal_bench {
    pub use crate::tunnel_core::p2p::route_table::{Route, RouteTable};
}

/// 版本号 = 基础版本（CARGO_PKG_VERSION）+ 子版本号。只递增末尾 X。
/// cargo 版本号只允许三段，因此把第四段子版本号拼接到展示字符串。
/// 子版本号优先取构建环境变量 VNT_SUBVERSION（CI 在 tag 构建时从
/// git tag 尾段注入，如 v2.0.10.4 -> "4"）；本地构建未设置时回退 "1"。
/// 各启动路径（CLI/web/JNI/桌面）统一经此上报版本。
pub const VERSION: &str = env!("VNT_VERSION");
