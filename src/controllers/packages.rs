use aws_creds::StaticProvider;
use config::S3 as s3_config;
use db;
use hab_core::package::{ident, PackageArchive, PackageIdent, PackageTarget};
use models::package::*;
use rocket::Data;
use rocket::Route;
use rocket::State;
use rocket::http::RawStr;
use rocket::response::{status, Failure, Stream};
use rocket_contrib::Json;
use std::fs::File;
use std::io;
use std::io::BufRead;
use tempdir::TempDir;
use rusoto_s3::{CreateBucketRequest, S3, S3Client};
use rusoto_core::Region;
use rusoto_core::reactor::RequestDispatcher;

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

#[get("/pkgs/search/<query>", rank = 1)]
fn search_packages(conn: db::DbConn, query: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>", rank = 2)]
fn list_packages_for_origin(conn: db::DbConn, origin: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/<origin>/pkgs", rank = 3)]
fn list_unique_packages(conn: db::DbConn, origin: &RawStr) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>", rank = 2)]
fn list_packages(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/versions", rank = 1)]
fn list_package_versions(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/latest", rank = 1)]
fn show_package_latest(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>", rank = 2)]
fn list_package_version(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>/latest", rank = 1)]
fn show_package_version_latest(
    conn: db::DbConn,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
) -> Result<Json<Package>, Failure> {
    unimplemented!()
}

#[get("/pkgs/<origin>/<pkg>/<version>/<release>", rank = 2)]
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
#[get("/pkgs/<origin>/<pkg>/<version>/download", rank = 1)]
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
    config: State<s3_config>,
    origin: &RawStr,
    pkg: &RawStr,
    version: &RawStr,
    release: &RawStr,
    data: Data,
) -> Result<status::Accepted<String>, Failure> {
    let dir = TempDir::new("tmphart").unwrap();
    let file_path = dir.path().join("temp.hart");
    let status = data.stream_to_file(file_path)
        .map(|n| n.to_string())
        .unwrap();
    let provider =
        StaticProvider::new_minimal(config.username.to_string(), config.password.to_string());
    let client = S3Client::new(RequestDispatcher::default(), provider, Region::UsWest2);
    let bucket_req = CreateBucketRequest {
        acl: None,
        bucket: config.bucket.to_string(),
        create_bucket_configuration: None,
        grant_full_control: None,
        grant_read: None,
        grant_read_acp: None,
        grant_write: None,
        grant_write_acp: None,
    };
    match client.create_bucket(&bucket_req).sync() {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    };
    Ok(status::Accepted(Some(status)))
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
