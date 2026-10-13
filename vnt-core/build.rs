fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();

    // 版本号 = 基础版本 + 子版本号：Cargo.toml 只允许三段 semver，第四段子
    // 版本号由构建环境变量 VNT_SUBVERSION 注入（CI 在 tag 构建时从 git tag
    // 尾段提取，如 v2.0.10.4 -> "4"），未设置时回退 "1"。拼接完整版本字符串
    // 到 VNT_VERSION，供代码 env!("VNT_VERSION") 使用。
    let sub_version =
        std::env::var("VNT_SUBVERSION").unwrap_or_else(|_| "1".to_string());
    let version = format!("{}.{}", env!("CARGO_PKG_VERSION"), sub_version);
    println!("cargo:rustc-env=VNT_VERSION={version}");
    println!("cargo:rerun-if-env-changed=VNT_SUBVERSION");

    match protoc_bin_vendored::protoc_bin_path() {
        Ok(protoc_path) => {
            config.protoc_executable(protoc_path);
        }
        Err(error) => {
            println!(
                "cargo:warning=vendored protoc unavailable ({error:?}); falling back to protoc from PATH"
            );
        }
    }

    config.protoc_arg("--experimental_allow_proto3_optional");
    // Keep selective-broadcast payloads reference counted. This avoids building an
    // intermediate Vec for every broadcast and lets decoding from `Bytes` retain a
    // slice of the received frame instead of copying the embedded packet.
    config.bytes([".protocol.control_message.SelectiveBroadcast.data"]);

    config.compile_protos(
        &[
            "proto/control_message.proto",
            "proto/rpc.proto",
            "proto/client.proto",
            "proto/fec.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
