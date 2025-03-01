// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::io::Result;

fn main() -> Result<()> {
    // let mut config = prost_build::Config::new();
    // config
    //     .out_dir("src/source")
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_protos(
    //         &["src/source/events.proto", "src/source/service.proto"],
    //         &["src/source/"],
    //     )?;

    // Compile source protos
    tonic_build::configure()
        .out_dir("src/source")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            ".events.Value.value",
            r#"#[serde(tag = "type", content = "value")]"#,
        )
        .field_attribute(".events.Map.values", r#"#[serde(flatten)]"#)
        .field_attribute(".events.List.values", r#"#[serde(flatten)]"#)
        .compile_protos(&["src/source/protos/service.proto"], &["src/source/protos"])?;

    // Compile registry protos
    tonic_build::configure()
        .out_dir("src/registry")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/registry.proto"], &["proto"])?;

    Ok(())
}
