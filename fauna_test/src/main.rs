use faunadb::prelude::*;
use futures::future::lazy;
use futures::future::Future;
use tokio::prelude;

fn main() {
    let client = Client::builder("secret").build_sync().unwrap();
    let query = Filter::new(
        Lambda::new("x", Gt::new(Var::new("x"), 2)),
        Array::from(vec![1, 2, 3]),
    );

    match client.query(query) {
        Ok(response) => println!("{:#?}", response),
        Err(error) => println!("Error: {:#?}", error),
    }
}
