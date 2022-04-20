use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="notes"]
pub struct Note {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    #[serde(skip_deserializing)]
    pub user: String
}

table! {
    notes (id) {
        id -> Nullable<Int4>,
        title -> Varchar,
        body -> Varchar,
        user -> Varchar,
    }
}