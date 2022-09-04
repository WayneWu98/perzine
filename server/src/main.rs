use perzine_server::core::AppState;
use perzine_server::route;
use rbatis::{self, Rbatis};
use rbdc_pg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rb = Rbatis::new();
    rb.init(
        rbdc_pg::driver::PgDriver {},
        "postgres://perzine:lll12345@101.132.156.196:5432/perzine",
    )
    .unwrap();

    let app = route::init(AppState { rb });
    axum::Server::bind(&"0.0.0.0:3031".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
