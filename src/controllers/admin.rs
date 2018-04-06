use rocket::http::RawStr;
use db;
use models::origin::*;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    return routes![admin_search, admin_account_show];
}

#[get("/admin/search")]
fn admin_search(conn: db::DbConn) -> Json<Vec<Origin>> {
    unimplemented!()
}

#[get("/admin/accounts/<id>")]
fn admin_account_show(conn: db::DbConn, id: i32) -> Json<Vec<Origin>> {
    unimplemented!()
}
