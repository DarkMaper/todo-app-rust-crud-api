use rocket::response::{Debug, status::{Created, NotFound}};
use rocket::serde::json::Json;

use crate::diesel::prelude::*;

type Result<T,E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use crate::models::note::*;
use crate::db::Db;
use uuid::Uuid;

use crate::guards::auth::authentication_guard::Token;

#[post("/create", data = "<note_data>")]
pub async fn create_note(db: Db, token: Token, note_data: Json<Note>) -> Result<Created<Json<Note>>> {
    let user = token.0.claims.sub;

    let note_values = note_data.clone();

    let new_note = Note {
        id: Some(Uuid::new_v4().to_string()),
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

#[get("/<id>")]
pub async fn get_note_info(db: Db, token: Token, id: String) -> Result<Json<Note>, NotFound<String>> {
    let user = token.0.claims.sub;

    let note = db.run(move |conn| {
        notes::table
            .filter(notes::user.eq(user).and(notes::id.eq(id)))
            .first(conn)
    }).await.map(Json);

    match note {
        Ok(data) => {
            Ok(data)
        },
        Err(_) => {
            Err(NotFound("No existe la nota".to_string()))
        }
    }
}

#[put("/update/<id>", data = "<note_data>")]
pub async fn update_note(db: Db, token: Token, id: String, note_data: Json<Note>) -> Result<Created<Json<Note>>, NotFound<String>> {

    let user = token.0.claims.sub;
    let note_values = note_data.clone();

    let affected = db.run(move |conn| {
        diesel::update(notes::table)
            .filter(notes::user.eq(user).and(notes::id.eq(id)))
            .set((notes::title.eq(note_values.title), notes::body.eq(note_values.body)))
            .execute(conn)
    }).await.ok();

    match affected {
        Some(data) => {
            if data > 0 {
                return Ok(Created::new("/").body(note_data));
            }
            
            Err(NotFound("No existe ninguna nota con ese ID".to_string()))
        },
        None => {
            Err(NotFound("No existe ninguna nota con ese ID".to_string()))
        }
    }
}

#[delete("/delete/<id>")]
pub async fn delete_note(db: Db, token: Token, id: String) -> Result<Option<()>, NotFound<String>> {
    let user = token.0.claims.sub;

    let affected = db.run(move |conn| {
        diesel::delete(notes::table)
            .filter(notes::user.eq(user).and(notes::id.eq(id)))
            .execute(conn)
    }).await.ok();

    match affected {
        Some(data) => {
            if data > 0 { 
                return Ok(Some(()));
            }

            Err(NotFound("No existe ninguna nota con ese id".to_string()))
            
        },
        None => Err(NotFound("No existe ninguna nota con ese id".to_string()))
    }
}