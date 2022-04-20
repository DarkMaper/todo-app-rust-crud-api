use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::diesel::prelude::*;

type Result<T,E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use crate::models::note::*;
use crate::db::Db;

use crate::guards::auth::authentication_guard::Token;

#[post("/create", data = "<note_data>")]
pub async fn create_note(db: Db, token: Token, note_data: Json<Note>) -> Result<Created<Json<Note>>> {
    let user = token.0.claims.sub;

    let note_values = note_data.clone();

    let new_note = Note {
        id: None,
        title: note_values.title,
        body: note_values.body,
        user: user
    };

    db.run(move |conn| {
        diesel::insert_into(notes::table)
            .values(&new_note)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(note_data))
}

#[get("/")]
pub async fn get_notes(db: Db, token: Token) -> Result<Json<Vec<Note>>> {
    let user = token.0.claims.sub;

    let notes = db.run(move |conn| {
        notes::table
            .filter(notes::user.eq(user))
            .load(conn)
    }).await?;

    Ok(Json(notes))
}