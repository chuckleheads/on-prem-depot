use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::origins;

#[derive(Debug, Serialize, Queryable)]
pub struct Origin {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub default_package_visibility: String,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origins"]
pub struct NewOrigin {
    pub name: String,
    pub owner_id: i64,
    pub default_package_visibility: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origins"]
pub struct UpdateOrigin {
    pub default_package_visibility: String,
}

impl Origin {
    pub fn insert(origin: &NewOrigin, conn: &PgConnection) -> bool {
        diesel::insert_into(origins::table)
        .values(origin)
        .execute(conn).is_ok()
    }
}