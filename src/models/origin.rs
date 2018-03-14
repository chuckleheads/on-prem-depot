use schema::origins;

#[derive(Debug, Serialize, Queryable)]
pub struct Origin {
    pub id: u64,
    pub name: String,
    pub owner_id: u64,
    pub session_sync: bool,
    pub default_package_visibility: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origins"]
pub struct NewOrigin {
    pub name: String,
    pub default_package_visibility: Option<String>,
}
