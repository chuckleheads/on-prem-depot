use db;
use models::origin::*;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    return routes![get_user_invitations, get_user_origins];
}

#[get("/user/invitations")]
fn get_user_invitations(conn: db::DbConn) -> Json<Vec<Origin>> {
    unimplemented!()
}

#[get("/user/origins")]
fn get_user_origins(conn: db::DbConn) -> Json<Vec<Origin>> {
    unimplemented!()
}
