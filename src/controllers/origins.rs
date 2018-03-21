use db;
use diesel;
use diesel::prelude::*;
use models::origin::*;
use rocket::http::RawStr;
use rocket::Route;
use rocket_contrib::Json;
use schema::origins;

pub fn routes() -> Vec<Route> {
    return routes![create_origin, update_origin];
}

#[post("/origins", format = "application/json", data = "<origin>")]
fn create_origin(conn: db::DbConn, origin: Json<NewOrigin>) -> Json<Origin> {
    let val = diesel::insert_into(origins::table)
        .values(&origin.into_inner())
        .get_result(&*conn)
        .expect("Error saving origin");
    println!("val = {:?}", &val);
    Json(val)
}

#[put("/origins/<name>")]
fn update_origin(conn: db::DbConn, name: &RawStr) -> Json<Origin> {
    unimplemented!()
}

#[get("/origins/<name>")]
fn get_origin(conn: db::DbConn, name: &RawStr) -> Json<Origin> {
    unimplemented!()
}

#[get("/origins/<origin>/keys")]
fn get_origin_keys(conn: db::DbConn, origin: &RawStr) -> Json<Vec<Origin>> {
    unimplemented!()
}
