use rocket::response::Failure;
use db;
use diesel::prelude::*;
use models::origin::*;
use rocket::http::{RawStr, Status};
use rocket::Route;
use rocket_contrib::Json;
use diesel::result::{DatabaseErrorKind, Error as diesel_error};

pub fn routes() -> Vec<Route> {
    routes![
        list_channels,
        create_channel,
        delete_channel,
        channel_packages,
        channel_package,
        channel_package_latest,
        channel_package_version,
        channel_package_version_latest,
        channel_package_version_release,
        channel_package_promote,
        channel_package_demote
    ]
}

#[get("/channels/<origin>")]
fn list_channels(conn: db::DbConn, origin: &RawStr) -> Json<Vec<String>> {
    unimplemented!()
}

#[post("/channels/<origin>/<channel>")]
fn create_channel(conn: db::DbConn, origin: &RawStr, channel: &RawStr) -> String {
    unimplemented!()
}

#[delete("/channels/<origin>/<channel>")]
fn delete_channel(conn: db::DbConn, origin: &RawStr, channel: &RawStr) -> String {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs")]
fn channel_packages(conn: db::DbConn, origin: &RawStr, channel: &RawStr) -> Json<Vec<String>> {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs/<pkg>")]
fn channel_package(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
) -> Json<Vec<String>> {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs/<pkg>/latest")]
fn channel_package_latest(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
) -> Json<String> {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs/<pkg>/<version>")]
fn channel_package_version(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
    version: &RawStr,
) -> Json<Vec<String>> {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs/<pkg>/<version>/latest")]
fn channel_package_version_latest(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
    version: &RawStr,
) -> Json<String> {
    unimplemented!()
}

#[get("/channels/<origin>/<channel>/pkgs/<pkg>/<version>/<release>")]
fn channel_package_version_release(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
    version: &RawStr,
    release: &RawStr,
) -> Json<String> {
    unimplemented!()
}

#[put("/channels/<origin>/<channel>/pkgs/<pkg>/<version>/<release>/promote")]
fn channel_package_promote(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
    version: &RawStr,
    release: &RawStr,
) -> Json<String> {
    unimplemented!()
}

#[put("/channels/<origin>/<channel>/pkgs/<pkg>/<version>/<release>/demote")]
fn channel_package_demote(
    conn: db::DbConn,
    origin: &RawStr,
    channel: &RawStr,
    package: &RawStr,
    version: &RawStr,
    release: &RawStr,
) -> Json<String> {
    unimplemented!()
}
