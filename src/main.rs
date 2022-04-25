#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

pub mod models;
pub mod db;
pub mod controllers;
pub mod guards;
pub mod lib;

use crate::controllers::user_controllers::*;
use crate::controllers::note_controllers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::stage())
    .mount("/auth", routes![signup, signin])
    .mount("/notes", routes![
        create_note, 
        get_notes, 
        get_note_info, 
        update_note,
        delete_note
        ])
    .attach(lib::cors::stage())
}
