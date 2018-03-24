use db;
use diesel;
use diesel::prelude::*;
use models::origin::*;
use rocket::http::RawStr;
use rocket::response::status;
use rocket::Route;
use rocket_contrib::Json;
use schema::origins;

pub fn routes() -> Vec<Route> {
    return routes![create_origin, update_origin, get_origin];
}

#[post("/origins", format = "application/json", data = "<origin>")]
fn create_origin(conn: db::DbConn, origin: Json<NewOrigin>) -> status::Created<Json<Origin>> {
    let name = &origin.name.clone();
    let val = diesel::insert_into(origins::table)
        .values(&origin.into_inner())
        .get_result(&*conn)
        .expect("Error saving origin");
    println!("val = {:?}", &val);

    // TODO: hardcoding this URL feels bad - surely there's a way to infer this?
    status::Created(format!("/origins/{}", name), Some(Json(val)))
}

#[put("/origins/<name>")]
fn update_origin(conn: db::DbConn, name: &RawStr) -> Json<Origin> {
    unimplemented!()
}

#[get("/origins/<origin>")]
fn get_origin(conn: db::DbConn, origin: &RawStr) -> Option<Json<Origin>> {
    use schema::origins::dsl::*;

    let mut o = origins
        .filter(name.eq(origin.percent_decode_lossy()))
        .limit(1)
        .load::<Origin>(&*conn)
        .expect("Error loading origins");

    if o.len() == 0 {
        None
    } else {
        Some(Json(o.pop().unwrap()))
    }
}

#[get("/origins/<origin>/keys")]
fn get_origin_keys(conn: db::DbConn, origin: &RawStr) -> Json<Vec<Origin>> {
    unimplemented!()
}
