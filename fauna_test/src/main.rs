use async_std;

#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    let string = surf::post("https://db.us.fauna.com")
        .header(
            http_types::headers::AUTHORIZATION,
            format!(
                "{} {}",
                http_types::auth::AuthenticationScheme::Bearer,
                include_str!("../../secret.txt")
            ),
        )
        .body("{\"get\":{\"collections\":null}}")
        .recv_string()
        .await?;
    println!("{}", string);
    Ok(())
}
