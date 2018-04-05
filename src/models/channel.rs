use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::{origin_channels, origins};
use super::origin::Origin as origin_model;

#[derive(Debug, Serialize, Queryable)]
pub struct Channel {
    pub id: i64,
    pub origin_id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_channels"]
pub struct NewChannel {
    pub name: String,
    pub owner_id: i64,
    pub origin_id: i64,
}

impl Channel {
    pub fn insert(channel: &NewChannel, conn: &PgConnection) -> QueryResult<Channel> {
        diesel::insert_into(origin_channels::table)
            .values(channel)
            .get_result(conn)
    }

    pub fn list(origin: &str, conn: &PgConnection) -> QueryResult<Vec<Channel>> {
        origins::table
            .inner_join(origin_channels::table)
            .filter(origins::name.eq(origin))
            .select(origin_channels::all_columns)
            .get_results(conn)
    }

    pub fn delete(origin: &str, channel: &str, conn: &PgConnection) -> QueryResult<usize> {
        let origin_id = origin_model::get(origin, conn).unwrap().unwrap().id;
        diesel::delete(
            origin_channels::table
                .filter(origin_channels::name.eq(channel))
                .filter(origin_channels::origin_id.eq(origin_id)),
        ).execute(conn)
    }
}
