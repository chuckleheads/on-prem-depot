use std::fs::File;
use std::io;

use db;
use hab_core::package::{ident, PackageArchive, PackageIdent, PackageTarget};
use models::package::*;
use rocket_contrib::Json;
use rocket::Data;
use rocket::http::RawStr;
use rocket::response::{Failure, Stream};
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        search_packages,
        list_packages_for_origin,
        list_unique_packages,
        list_packages,
        list_package_versions,
        show_package_latest,
        list_package_version,
        show_package_version_latest,
        show_package_version_release,
        package_channels,
        download_package,
        upload_package,
        package_privacy_toggle,
    ]
}

#[get("/pkgs/search/<query>")]
fn search_packages(conn: db::DbConn, query: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>")]
fn list_packages_for_origin(conn: db::DbConn, origin: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/<origin>/pkgs")]
fn list_unique_packages(conn: db::DbConn, origin: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>")]
fn list_packages(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/versions")]
fn list_package_versions(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/latest")]
fn show_package_latest(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>")]
fn list_package_version(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>/latest")]
fn show_package_version_latest(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>/<release>")]
fn show_package_version_release(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
    release: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>/<release>/channels")]
fn package_channels(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
    release: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

// TODO: probably need to return a binary stream here
#[get("/pkgs/<origin>/<pkg>/<version>/download")]
fn download_package(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
) -> io::Result<Stream<File>> {
    unimplemented!()
}

#[post("/pkgs/<origin>/<pkg>/<version>/<release>", format = "application/octet-stream",
       data = "<data>")]
fn upload_package(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
    release: &RawStr,
    data: Data,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[patch("/pkgs/<origin>/<pkg>/<version>/<release>/<visibility>")]
fn package_privacy_toggle(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
    release: &RawStr,
    visibility: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}
