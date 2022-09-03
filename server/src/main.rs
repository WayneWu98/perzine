use dotenv::dotenv;

use perzine_server::core::{AppConfig, AppState};
use perzine_server::route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cfg = AppConfig::from_env().expect("App initialize fail!");
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
