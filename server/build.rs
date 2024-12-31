fn main() -> anyhow::Result<()> {
    wasm_pack::command::build::Build::run(&mut wasm_pack::command::build::Build::try_from_opts(
        wasm_pack::command::build::BuildOptions {
            target: wasm_pack::command::build::Target::Web,
            path: Some("../client".into()),
            out_dir: "../client_dist".into(),
            ..Default::default()
        },
    )?)?;

    let client_js_binary = std::fs::read("../client_dist/client.js")?;
    let client_js = std::str::from_utf8(&client_js_binary)?;

    let wasm_hash_value: String = {
        use sha2::Digest;
        let client_bg_wasm_binary = std::fs::read("../client_dist/client_bg.wasm")?;

        let mut sha256 = sha2::Sha256::new();
        sha256.update(client_bg_wasm_binary);
        format!("{:x}", sha256.finalize())
    };

    let replaced_client_js = client_js.replace("client_bg.wasm", &wasm_hash_value);
    std::fs::write("../client_dist/client.js", replaced_client_js.clone())?;
    let replaced_client_js_clone = replaced_client_js.clone();

    std::fs::write(
        "../client_dist/client_bg.wasm.hash.txt",
        format!("/{}", wasm_hash_value),
    )?;
    std::fs::write("../client_dist/client.js.hash.txt", {
        use sha2::Digest;

        let mut sha256 = sha2::Sha256::new();
        sha256.update(replaced_client_js_clone);
        format!("/{:x}", sha256.finalize())
    })?;

    std::fs::create_dir_all("../client_dist/assets")?;

    std::fs::read_dir("../assets")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .try_for_each(|path| {
            let file_name = &path.file_name().unwrap().to_str().unwrap();
            std::fs::write(format!("../client_dist/assets/{}.hash.txt", file_name), {
                use sha2::Digest;
                let client_bg_wasm_binary = std::fs::read(&path)?;

                let mut sha256 = sha2::Sha256::new();
                sha256.update(client_bg_wasm_binary);
                format!("/{:x}", sha256.finalize())
            })
        })?;

    Ok(())
}
