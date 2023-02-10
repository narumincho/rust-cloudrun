use std::str::FromStr;

use serde_json::json;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = request(
        &include_str!("../../secret.txt"),
        &Expr::Collection(Box::new(Expr::StringLiteral("aaa".to_string()))),
    )
    .await?;
    println!("{}", result);
    Ok(())
}

async fn request(secret_key: &str, query: &Expr) -> anyhow::Result<serde_json::value::Value> {
    let client = reqwest::Client::new();
    let string = client
        .post("https://db.us.fauna.com")
        .bearer_auth(secret_key)
        .json(&expr_to_json_value(query))
        .send()
        .await?
        .text()
        .await?;
    let json_value = serde_json::value::Value::from_str(string.as_str())?;
    Ok(json_value)
}

enum Expr {
    Get(Box<Expr>),
    Collections,
    Collection(Box<Expr>),
    StringLiteral(String),
}

fn expr_to_json_value(expr: &Expr) -> serde_json::value::Value {
    match expr {
        Expr::Get(ref_expr) => json!({ "get": expr_to_json_value(ref_expr) }),
        Expr::Collections {} => json!({ "collections": null }),
        Expr::Collection(name_expr) => json!({ "collection": expr_to_json_value(name_expr) }),
        Expr::StringLiteral(value) => serde_json::value::Value::String(value.clone()),
    }
}
