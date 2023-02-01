use async_std;
use serde_json::json;

#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    let result = request(
        &include_str!("../../secret.txt"),
        &Expr::Get(Box::new(Expr::Collections)),
    )
    .await?;
    println!("{}", result);
    Ok(())
}

async fn request(secret_key: &str, query: &Expr) -> Result<String, http_types::Error> {
    let string = surf::post("https://db.us.fauna.com")
        .header(
            http_types::headers::AUTHORIZATION,
            format!(
                "{} {}",
                http_types::auth::AuthenticationScheme::Bearer,
                secret_key
            ),
        )
        .body(expr_to_json_value(&query))
        .recv_string()
        .await?;
    Ok(string)
}

enum Expr {
    Get(Box<Expr>),
    Collections,
}

fn expr_to_json_value(expr: &Expr) -> serde_json::value::Value {
    match expr {
        Expr::Get(ref_expr) => json!({ "get": expr_to_json_value(ref_expr) }),
        Expr::Collections {} => json!({ "collections": null }),
    }
}
