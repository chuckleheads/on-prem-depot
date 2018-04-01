use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::{origin_public_keys, origin_secret_keys};

// Secret Signing Keys

#[derive(Debug, Serialize, Queryable)]
pub struct OriginSecretKey {
    pub id: i64,
    pub owner_id: i64,
    pub origin_id: i64,
    pub name: String,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_secret_keys"]
pub struct NewOriginSecretKey {
    pub name: String,
    pub owner_id: i64,
    pub origin_id: i64,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_secret_keys"]
pub struct UpdateOriginSecretKey {
    pub name: String,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
}

impl OriginSecretKey {
    pub fn insert(
        origin_secret_key: &NewOriginSecretKey,
        conn: &PgConnection,
    ) -> QueryResult<OriginSecretKey> {
        diesel::insert_into(origin_secret_keys::table)
            .values(origin_secret_key)
            .get_result(conn)
    }

    pub fn update(osk: UpdateOriginSecretKey, conn: &PgConnection) -> QueryResult<OriginSecretKey> {
        use schema::origin_secret_keys::dsl::{name as osk_name, *};
        diesel::update(origin_secret_keys)
            .set((
                osk_name.eq(&osk.name),
                revision.eq(&osk.revision),
                full_name.eq(&osk.full_name),
                body.eq(&osk.body),
            ))
            .get_result(conn)
    }

    pub fn get(name: &str, conn: &PgConnection) -> QueryResult<Option<OriginSecretKey>> {
        use schema::origin_secret_keys::dsl::{name as osk_name, origin_secret_keys};
        origin_secret_keys
            .filter(osk_name.eq(name))
            .limit(1)
            .get_result(conn)
            .optional()
    }
}

// Public Signing Keys

#[derive(Debug, Serialize, Queryable)]
pub struct OriginPublicKey {
    pub id: i64,
    pub owner_id: i64,
    pub origin_id: i64,
    pub name: String,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
    pub created_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_public_keys"]
pub struct NewOriginPublicKey {
    pub name: String,
    pub owner_id: i64,
    pub origin_id: i64,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origin_public_keys"]
pub struct UpdateOriginPublicKey {
    pub name: String,
    pub revision: String,
    pub full_name: String,
    pub body: Vec<u8>,
}

impl OriginPublicKey {
    pub fn insert(
        origin_public_key: &NewOriginPublicKey,
        conn: &PgConnection,
    ) -> QueryResult<OriginPublicKey> {
        diesel::insert_into(origin_public_keys::table)
            .values(origin_public_key)
            .get_result(conn)
    }

    pub fn get(name: &str, conn: &PgConnection) -> QueryResult<Option<OriginPublicKey>> {
        use schema::origin_public_keys::dsl::{name as opk_name, origin_public_keys};
        origin_public_keys
            .filter(opk_name.eq(name))
            .limit(1)
            .get_result(conn)
            .optional()
    }
}
