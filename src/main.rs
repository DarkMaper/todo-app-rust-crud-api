#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;


pub mod models;

pub mod db;
pub mod controllers;
pub mod guards;
pub mod lib;
pub mod catchers;

use crate::controllers::user_controllers::*;
use crate::controllers::note_controllers::*;


use crate::catchers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::stage())
    .mount("/api/auth", routes![signup, signin, refresh_token, logout])
    .mount("/api/notes", routes![
        create_note, 
        get_notes, 
        get_note_info, 
        update_note,
        delete_note
        ])
    .register("/", catchers![unauthorized])
    .attach(lib::cors::stage())
}
