use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="sessions"]
pub struct Session {
    #[serde(skip_deserializing)]
    pub token: Option<String>,
    #[serde(skip_deserializing)]
    pub user: String
}

table! {
    sessions (token) {
        token -> Nullable<Varchar>,
        user -> Varchar,
    }
}