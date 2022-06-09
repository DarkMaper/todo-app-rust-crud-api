use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="notes"]
pub struct Note {
    #[serde(skip_deserializing)]
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    #[serde(skip_deserializing)]
    pub user: String
}

table! {
    notes (id) {
        id -> Nullable<Varchar>,
        title -> Varchar,
        content -> Varchar,
        user -> Varchar,
    }
}