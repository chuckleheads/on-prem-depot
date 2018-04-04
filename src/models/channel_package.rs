// use diesel;
// use diesel::PgConnection;
// use diesel::prelude::*;
// use chrono::NaiveDateTime;
// use schema::origin_channels;

// #[derive(Debug, Serialize, Queryable)]
// pub struct ChannelPackage {
//     pub id: i64,
//     pub origin_id: i64,
//     pub owner_id: i64,
//     pub name: String,
//     pub created_at: Option<NaiveDateTime>,
//     pub update_at: Option<NaiveDateTime>,
// }

// #[derive(Debug, Deserialize, Insertable)]
// #[table_name = "origin_channels"]
// pub struct NewChannelPackage {
//     pub name: String,
//     pub owner_id: i64,
//     pub default_package_visibility: Option<String>,
// }

// #[derive(Debug, Deserialize, Insertable)]
// #[table_name = "origin_channels"]
// pub struct UpdateChannelPackage {
//     pub default_package_visibility: String,
// }

// impl Channel {
//     pub fn insert(channel: &NewChannel, conn: &PgConnection) -> QueryResult<Channel> {
//         diesel::insert_into(origin_channels::table)
//             .values(channel)
//             .get_result(conn)
//     }

//     pub fn list(origin: &str, channel: &str, conn: &PgConnection) -> QueryResult<Vec<Channel>> {
//         unimplemented!()
//     }

//     pub fn delete(origin: &str, channel: &str, conn: &PgConnection) -> QueryResult<Channel> {
//         unimplemented!()
//     }
// }
