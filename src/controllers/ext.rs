use rocket::http::RawStr;
use db;
use models::origin::*;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    return routes![validate_registry_credentials];
}

#[get("/ext/integrations/<registry_type>/credentials/validate")]
fn validate_registry_credentials(conn: db::DbConn, registry_type: &RawStr) -> Json<Vec<Origin>> {

    unimplemented!()
}
