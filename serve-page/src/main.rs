use std::error::Error;
use axum::Router;
use axum::routing::get;
use axum::body::{BoxBody, boxed};
use axum::http::{Request, StatusCode, Uri};
use axum::response::Response;
use tower_http::services::ServeDir;
use tower::util::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/*path", get(file_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}


async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(axum::body::Body::empty()).unwrap();

    match ServeDir::new("./client").oneshot(req).await {
        Ok(res) => {
            let response = res.map(boxed);
            print!("response: {:?}", response);
            Ok(response)
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
