pub fn main() -> anyhow::Result<()> {
    wasm_pack::command::build::Build::run(&mut wasm_pack::command::build::Build::try_from_opts(
        wasm_pack::command::build::BuildOptions {
            target: wasm_pack::command::build::Target::Web,
            path: Some("./client".into()),
            out_dir: "./pkg".into(),
            ..Default::default()
        },
    )?)?;

    let client_js_binary = std::fs::read("./client/pkg/client.js")?;
    let client_js = std::str::from_utf8(&client_js_binary)?;

    let wasm_hash_value: String = {
        use sha2::Digest;
        let client_bg_wasm_binary = std::fs::read("./client/pkg/client_bg.wasm")?;

        let mut sha256 = sha2::Sha256::new();
        sha256.update(client_bg_wasm_binary);
        format!("{:x}", sha256.finalize())
    };

    let replaced_client_js = client_js.replace("client_bg.wasm", &wasm_hash_value);
    std::fs::write("./client/pkg/client.js", replaced_client_js)?;
    Ok(())
}
