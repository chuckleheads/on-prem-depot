use rocket::request::Request;
use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::origins;
use rocket::response::{self, Responder, Response};
use rocket::http::ContentType;
use serde_json;

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
    pub fn insert(origin: &NewOrigin, conn: &PgConnection) -> QueryResult<Origin> {
        diesel::insert_into(origins::table)
            .values(origin)
            .get_result(conn)
    }

    pub fn update(name: &str, dpv: UpdateOrigin, conn: &PgConnection) -> QueryResult<Origin> {
        use schema::origins::dsl::{default_package_visibility, name as origin_name, origins};
        diesel::update(origins.filter(origin_name.eq(name)))
            .set(default_package_visibility.eq(&dpv.default_package_visibility))
            .get_result(conn)
    }

    pub fn get(name: &str, conn: &PgConnection) -> QueryResult<Option<Origin>> {
        use schema::origins::dsl::{name as origin_name, origins};
        origins
            .filter(origin_name.eq(name))
            .limit(1)
            .get_result::<Origin>(conn)
            .optional()
    }
}
