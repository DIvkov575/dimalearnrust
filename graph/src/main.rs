use axum::{extract, routing::get, Router};
use serde::Deserialize;

// #[derive(Deserialize)]
// struct Data {
//     name: String,
// }
// async fn parse_data(extract::Json(data)) {
//     let a: Data = data;
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", get(|| async { "Hello, World!" }));
//         // .route("/", post(parse_data);
//
//     // run it with hyper on localhost:3000
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// fn main() {
//     let mut a: Box<i32>= Box::new(25);
//     let b = a;
//     let c = &b;
//     a = b;
//     println!("{}", a);
//     // println!("{}", b);
//     // println!("{}", c);
//
// }
