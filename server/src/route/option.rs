use crate::route::AppResponse;
use crate::model::site_option::SiteOption;

pub async fn get_options() -> AppResponse<Vec<SiteOption>> {
    let options = SiteOption::all(pool)
}