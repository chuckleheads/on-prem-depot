use db;
use models::origin::*;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    return routes![get_profile, get_profile_access_tokens, post_profile_access_tokens];
}

#[get("/profile")]
fn get_profile(conn: db::DbConn) -> Json<Vec<Origin>> {

    unimplemented!()
}

#[get("/profile/access_tokens")]
fn get_profile_access_tokens(conn: db::DbConn) -> Json<Vec<Origin>> {

    unimplemented!()
}

#[post("/profile/access_tokens")]
fn post_profile_access_tokens(conn: db::DbConn) -> Json<Vec<Origin>> {

    unimplemented!()
}

#[delete("/profile/access_tokens/:id")]
fn delete_profile_access_tokens(conn: db::DbConn) -> Json<Vec<Origin>> {

    unimplemented!()
}