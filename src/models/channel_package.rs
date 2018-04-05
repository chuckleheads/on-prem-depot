use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::{origin_channel_packages, origins};

#[derive(Debug, Serialize, Queryable)]
pub struct ChannelPackage {
    pub channel_id: i64,
    pub package_id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_channel_packages"]
pub struct NewChannelPackage {
    pub channel_id: i64,
    pub package_id: i64,
}

// can't implement this until we implement Package

// impl ChannelPackage {
//     pub fn get(
//         origin: &str,
//         channel: &str,
//         pkg: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Package> {
//         unimplemented!()
//     }

//     pub fn insert(channel: &NewChannel, conn: &PgConnection) -> QueryResult<ChannelPackage> {
//         diesel::insert_into(origin_channels::table)
//             .values(channel)
//             .get_result(conn)
//     }

//     pub fn list(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }

//     pub fn latest(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }

//     pub fn version(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         version: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }

//     pub fn version_latest(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         version: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }

//     pub fn version_release(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         version: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }

//     pub fn promote(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         version: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }
//     pub fn demote(
//         origin: &str,
//         channel: &str,
//         package: &str,
//         version: &str,
//         conn: &PgConnection,
//     ) -> QueryResult<Vec<Package>> {
//         unimplemented!()
//     }
// }
