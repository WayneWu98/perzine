use perzine_server::core::{AppState, PGConfig, APP_CONFIG};
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_conf = &APP_CONFIG.server;
    let PGConfig {
        username,
        host,
        password,
        db,
    } = &APP_CONFIG.pg;
    let mut opt = ConnectOptions::new(
        format!("postgres://{}:{}@{}/{}", username, password, host, db).to_owned(),
    );
    opt.min_connections(8).max_connections(16);
    let db = Database::connect(opt).await?;
    let app = perzine_server::route::init(AppState { db });
    axum::Server::bind(&server_conf.addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
