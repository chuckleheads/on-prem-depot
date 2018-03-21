use schema::origins;

#[derive(Debug, Serialize, Queryable)]
pub struct Origin {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub session_sync: bool,
    pub default_package_visibility: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origins"]
pub struct NewOrigin {
    pub name: String,
    pub default_package_visibility: String,
}
