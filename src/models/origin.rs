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
    pub default_package_visibility: String,
}
