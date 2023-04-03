pub fn main() -> anyhow::Result<()> {
    let output = std::process::Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "sh"
    })
    .arg("wasm-pack")
    .arg("build")
    .arg("./client")
    .arg("--target")
    .arg("web")
    .output()?;

    println!("stdout: {}", std::str::from_utf8(&output.stdout)?);
    println!("stderr: {}", std::str::from_utf8(&output.stderr)?);

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
