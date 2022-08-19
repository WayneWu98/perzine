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

use perzine_server::{
    model::{self, site_option::SiteOption},
    route,
    utils::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cfg = perzine_server::utils::AppConfig::from_env().expect("App initialize fail!");
    let pool = cfg
        .pg
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    let app = route::init(AppState { pool });
    axum::Server::bind(&"0.0.0.0:3031".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
