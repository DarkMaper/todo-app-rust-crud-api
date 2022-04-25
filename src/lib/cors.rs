use rocket::fairing::AdHoc;
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn stage() -> AdHoc {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:8080"]),
        ..Default::default()
    }
    .to_cors().unwrap();

    AdHoc::on_ignite("Added CORS to responses", |rocket| async {
        rocket
            .attach(cors)
    })
}