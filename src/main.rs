#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate chrono;
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate habitat_core as hab_core;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rusoto_core;
extern crate rusoto_credential as aws_creds;
extern crate rusoto_s3;
extern crate serde_json;
extern crate tempdir;
extern crate toml;
extern crate url;
pub mod config;
pub mod controllers;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub mod types;
pub use self::config::{Config, CFG_DEFAULT_PATH};
pub use self::error::{Error, Result};
use std::io::stdout;

use clap::{App, Arg, SubCommand};

embed_migrations!("migrations");

fn main() {
    let matches = app().get_matches();
    let config = match matches.value_of("config") {
        Some(cfg_path) => Config::from_file(cfg_path).unwrap(),
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    let conn = db::init_pool(config.db);
    let handle = conn.get().expect("Couldn't get a db handle");
    embedded_migrations::run_with_output(&*handle, &mut stdout()).unwrap();
    rocket::ignite()
        .mount("/", controllers::routes())
        .catch(error::error_handlers())
        .manage(conn)
        .manage(config.s3)
        .launch();
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Habitat Builder")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("start").about("Start Habitat Builder"))
}
