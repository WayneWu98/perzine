use perzine_server::core::{AppState, APP_CONFIG};
use perzine_server::route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = APP_CONFIG.clone();
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
