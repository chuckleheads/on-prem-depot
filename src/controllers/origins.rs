use diesel;
use db;
use models::origin::*;
use diesel::prelude::*;
use rocket::Route;
use rocket_contrib::Json;

pub fn routes() -> Vec<Route> {
    return routes![create_origin, update_origin];
}

#[post("/origins", format = "application/json", data = "<origin>")]
fn create_origin(conn: db::DbConn, origin: Json<NewOrigin>) -> Json<Vec<Origin>> {
    unimplemented!()
}

#[put("/origins/:name")]
fn update_origin(conn: db::DbConn) -> Json<Origin> {
    unimplemented!()
}

#[get("/origins/:name")]
fn get_origin(conn: db::DbConn) -> Json<Origin> {
    unimplemented!()
}

#[get("/origins/:origin/keys")]
fn get_origin_keys(conn: db::DbConn) -> Json<Vec<Origin>> {
    unimplemented!()
}