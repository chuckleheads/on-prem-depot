use rocket::response::Failure;
use db;
use diesel::prelude::*;
use models::origin::*;
use rocket::http::{RawStr, Status};
use rocket::Route;
use rocket_contrib::Json;
use diesel::result::{DatabaseErrorKind, Error as diesel_error};

pub fn routes() -> Vec<Route> {
    return routes![create_origin, update_origin, get_origin];
}

#[post("/origins", format = "application/json", data = "<origin>")]
fn create_origin(conn: db::DbConn, origin: Json<NewOrigin>) -> Result<Json<Origin>, Failure> {
    match Origin::insert(&origin.into_inner(), &*conn) {
        Ok(o) => Ok(Json(o)),
        Err(diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err(Failure(Status::Conflict))
        }
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

#[put("/origins/<name>", format = "application/json", data = "<pacakge_visibility>")]
fn update_origin(
    conn: db::DbConn,
    name: &RawStr,
    pacakge_visibility: Json<UpdateOrigin>,
) -> QueryResult<Json<Origin>> {
    Origin::update(
        &name.percent_decode_lossy(),
        pacakge_visibility.into_inner(),
        &*conn,
    ).map(|origin| Json(origin))
}

#[get("/origins/<origin>")]
fn get_origin(conn: db::DbConn, origin: &RawStr) -> Result<Option<Json<Origin>>, Failure> {
    match Origin::get(&origin.percent_decode_lossy(), &*conn) {
        Ok(Some(v)) => Ok(Some(Json(v))),
        Ok(None) => Ok(None),
        Err(_) => Err(Failure(Status::InternalServerError)),
    }
}

#[get("/origins/<origin>/keys")]
fn get_origin_keys(conn: db::DbConn, origin: &RawStr) -> Json<Vec<Origin>> {
    unimplemented!()
}
