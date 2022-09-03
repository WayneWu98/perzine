use dotenv::dotenv;
use std::error::Error;

use perzine_server::{route, utils::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cfg = perzine_server::utils::AppConfig::from_env().expect("App initialize fail!");
    let pool = cfg.pg.create_pool(
        Some(deadpool_postgres::Runtime::Tokio1),
        tokio_postgres::NoTls,
    )?;
    let app = route::init(AppState { pool });
    axum::Server::bind(&"0.0.0.0:3031".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
