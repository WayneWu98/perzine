use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::*,
    Extension, Json, Router,
};
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, option, sync::Arc};
use tokio_postgres::NoTls;

use perzine_server::model::{self, site_option::SiteOption};

#[derive(Deserialize, Serialize)]
struct Response<T: IntoResponse> {
    code: usize,
    data: Option<T>,
    message: String,
}

impl<T: IntoResponse> Response<T> {
    fn new(data: Option<T>) -> Self {
        Self {
            code: 200,
            data,
            message: "ok".to_string(),
        }
    }
}

async fn get_index(Query(params): Query<HashMap<String, String>>) -> Json<Response<()>> {
    Json(Response::new(Some(())))
}

async fn get_option(
    Extension(state): Extension<Arc<State>>,
    Path(name): Path<String>,
) -> Json<Response<String>> {
    let value = match SiteOption::from_name(&state.pool, &name).await {
        Ok(option) => option.value,
        _ => String::from(""),
    };
    Json(Response::new(Some(value)))
}

struct State {
    pool: Pool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cfg = perzine_server::utils::AppConfig::from_env().expect("App initialize fail!");
    let pool = cfg
        .pg
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    let options = SiteOption::all(&pool).await?;
    println!("Options: {:?}", options);
    let app = Router::new()
        .route("/", get(get_index))
        .route("/options", get(get_option))
        .nest("/:optoin", get(get_option))
        .layer(Extension(Arc::new(State { pool })));
    axum::Server::bind(&"0.0.0.0:3031".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
