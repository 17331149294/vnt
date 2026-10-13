fn main() {
    // 版本号 = 基础版本 + 子版本号：Cargo.toml 只允许三段 semver，第四段子
    // 版本号由构建环境变量 VNT_SUBVERSION 注入（CI 在 tag 构建时从 git tag
    // 尾段提取，如 v2.0.10.4 -> "4"），未设置时回退 "1"。拼接完整版本字符串
    // 到 VNT_VERSION，供代码 env!("VNT_VERSION") 使用。
    let sub_version = std::env::var("VNT_SUBVERSION").unwrap_or_else(|_| "1".to_string());
    let version = format!("{}.{}", env!("CARGO_PKG_VERSION"), sub_version);
    println!("cargo:rustc-env=VNT_VERSION={version}");
    println!("cargo:rerun-if-env-changed=VNT_SUBVERSION");
}
