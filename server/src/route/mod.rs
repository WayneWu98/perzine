pub mod auth;
pub mod option;

use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use axum::routing::post;
use axum::Extension;
use axum::{routing::get, Router};

use crate::core::response::ResponseBody;
use crate::core::AppState;

use self::option::get_options;

// async fn propagate_header<B>(
//     req: axum::http::Request<B>,
//     next: axum::middleware::Next<B>,
// ) -> Response {
//     let mut res = next.run(req).await;
//     let body: ResponseBody<()> = res.body_mut().try_into();
//     match HeaderValue::from_str("hello") {
//         Ok(v) => res.headers_mut().insert("custom-key", v),
//         _ => None,
//     };
//     res
// }

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/options", get(get_options))
        .route("/login", post(auth::login))
        .route("/protected", get(auth::protected))
        .layer(Extension(std::sync::Arc::new(state)))
    // .layer(axum::middleware::from_fn(propagate_header))
}
