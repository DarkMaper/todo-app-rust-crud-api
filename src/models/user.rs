use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="users"]
pub struct User {
    pub email: String,
    pub password: String
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
    }
}