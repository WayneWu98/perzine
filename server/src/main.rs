use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use std::error::Error;
use tokio_postgres::NoTls;

use perzine_server::models::site_option::SiteOption;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cfg = perzine_server::utils::AppConfig::from_env().expect("App initialize fail!");
    // println!("app config: {:#?}", cfg);
    // let pool = cfg
    //     .pg
    //     .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
    //     .expect("Perzine initialize fail.");
    // println!("got pool");
    let pool = cfg
        .pg
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    let site_name = SiteOption::from_name(&pool, "site_name").await?;
    println!("option.site_name = {:#?}", site_name);
    Ok(())
}
