pub fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=../client/src");
    std::fs::write("./out.txt", "test!!??")?;
    Ok(())
}
