#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod models;
pub mod db;
pub mod controllers;
pub mod guards;

use crate::guards::auth::authentication_guard::*;
use crate::controllers::user_controllers::*;
use crate::controllers::note_controllers::*;

#[get("/")]
fn index(token: Token) -> String {
    
    format!("Welcome {}.", token.0.claims.sub)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::stage())
    .mount("/", routes![index])
    .mount("/auth", routes![signup, signin])
    .mount("/notes", routes![create_note, get_notes])
}
