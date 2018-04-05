use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::{origin_packages, origins};

#[derive(Debug, Serialize, Queryable)]
pub struct Package {
    pub id: i64,
    pub origin_id: i64,
    pub owner_id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_packages"]
pub struct NewPackage {
    pub origin_id: i64,
    pub owner_id: i64,
    pub name: String,
    pub ident: String,
    pub checksum: String,
    pub manifest: String,
    pub config: String,
    pub target: String,
    pub deps: String,
    pub tdeps: String,
    pub exposes: String,
    pub visibility: String,
}

impl Package {}
