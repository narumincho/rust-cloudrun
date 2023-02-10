pub fn main() -> anyhow::Result<()> {
    let output = std::process::Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "sh"
    })
    .arg("wasm-pack")
    .arg("build")
    .arg("./client")
    .arg("--target").arg("web")
    .output()?;

    println!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap_or("UIF8として解釈できず"));
    println!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap_or("UIF8として解釈できず"));
    Ok(())
}
