use rocket::fairing::AdHoc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;

pub fn stage() -> AdHoc {
    let allow_origins: String = env::var("ALLOWED_ORIGINS").unwrap();
    let allow_origins_vec: Vec<&str> = allow_origins.split(",").collect();
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&allow_origins_vec),
        ..Default::default()
    }
    .to_cors().unwrap();

    AdHoc::on_ignite("Added CORS to responses", |rocket| async {
        rocket
            .attach(cors)
    })
}