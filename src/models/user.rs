use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<String>,
    pub username: String,
    pub password: String
}

table! {
    users (id) {
        id -> Nullable<Varchar>,
        username -> Varchar,
        password -> Varchar,
    }
}