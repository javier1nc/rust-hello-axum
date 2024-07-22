use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))//.route("/", get(|| async { "Hello, World!" }));
        .route("/plain_text", get(plain_text).post(post_foo))
        .route("/json", get(json))
        .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // which calls one of these handlers
    async fn root() -> &'static str {
        "Hello, World!"
    }
    // `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
    async fn plain_text() -> &'static str {
        "foo"
    }

    // `Json` gives a content-type of `application/json` and works with any type
    // that implements `serde::Serialize`
    async fn json() -> Json<Value> {
        Json(json!(
            {
                "status": "success",
                "data":[
                    {
                        "id":1,
                        "image_name":"javier1nc",
                        "image_url":"srtrstrst"
                    }
                ]
            }
        ))
    }
    async fn post_foo() {}
    async fn foo_bar() {}
}