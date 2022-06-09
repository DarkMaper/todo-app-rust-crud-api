use rocket::response::{Debug, status::{Created, Conflict, Unauthorized}};
use rocket::http::{CookieJar, Cookie};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use bcrypt::{hash, verify, DEFAULT_COST};

use chrono::prelude::*;

use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;

use crate::diesel::prelude::*;

type Result<T,E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use crate::models::user::*;
use crate::models::session::*;
use crate::db::Db;

fn generate_jwt(user: Json<User>) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.clone().unwrap(),
        exp: expiration as usize,
        username: user.username.clone()
    };

    return encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap();
}

async fn check_user_exists(db: &Db, username: String) -> Option<Json<User>> {

    let user = db.run(move |conn| {
        users::table
            .filter(users::username.eq(username.to_owned()))
            .first(conn)
    }).await.map(Json).ok();

    match user {
        Some(usr) => {
            Some(usr)
        },
        None => {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JWT {
    pub access_token: String
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
    pub username: String,
}

#[post("/signup", data = "<signin>")]
pub async fn signup(db: Db, signin: Json<User>) -> Result<Created<String>, Conflict<String>> {

    let user_value = signin.clone();

    let check_user = check_user_exists(&db, user_value.username.clone()).await;

    match check_user {
        Some(_) => {
            return Err(Conflict(Some("Email already in use".to_string())));
        },
        None => {
            let hashed_password = hash(&user_value.password, DEFAULT_COST).unwrap();
            let new_user = User {
                id: Some(Uuid::new_v4().to_string()),
                username: user_value.username.clone(),
                password: hashed_password
            };

            let affected = db.run(move |conn| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .execute(conn)
            }).await.ok();
                
            match affected
            {
                Some(data) => {
                    println!("{:?}", data);
                    return Ok(Created::new("/").body("User created".to_string()));
                },
                None => {
                    return Err(Conflict(Some("Email already in use".to_string())));
                }
            }
        }
    };
}

#[post("/signin", data = "<login_form>")]
pub async fn signin(db: Db, login_form: Json<User>, cookies: &CookieJar<'_>) -> Result<Json<JWT>, Unauthorized<String>> {
    let user_value = login_form.clone();

    let check_user = check_user_exists(&db, user_value.username.clone()).await;
    
    match check_user { 
        Some(user) => {
            let password_match = verify(user_value.password, &user.password);

            match password_match {
                Ok(result) => {
                    if !result { 
                        return Err(Unauthorized(Some("Email or password are incorrect".to_string())));
                    }

                    let refresh_token = Uuid::new_v4().to_string();

                    let new_session = Session {
                        token: Some(refresh_token.clone()),
                        user:  user.id.clone().unwrap()
                    };
        
                    db.run(move |conn| {
                        diesel::insert_into(sessions::table)
                            .values(new_session)
                            .execute(conn)
                    }).await.ok();

                    let access_token = generate_jwt(user);

                    cookies.add(Cookie::build("session", refresh_token).http_only(true).finish());

                    let jwt = JWT{access_token};

                    Ok(Json(jwt))
                },
                Err(_) => {
                    return Err(Unauthorized(Some("Email or password are incorrect".to_string())));
                }
            }
        },
        None => {
            return Err(Unauthorized(Some("Email or password are incorrect".to_string())));
        }
    }

}

#[get("/refresh")]
pub async fn refresh_token(db: Db, cookies: &CookieJar<'_>) -> Result<Json<JWT>, Unauthorized<String>> {
    let session_token = cookies.get("session");

    match session_token {
        Some(session_tkn) => {
            let session_value = session_tkn.value().to_string();

            let session_db = db.run(move |conn| {
                sessions::table
                    .filter(sessions::token.eq(session_value))
                    .first(conn)
            }).await.map(Json::<Session>).ok();

            match session_db {
                Some(session) => {
                    let user = db.run(move |conn| {
                        users::table
                            .filter(users::id.eq(session.user.clone()))
                            .first(conn)
                    }).await.map(Json).ok();

                    match user {
                        Some(user) => {
                            let access_token = generate_jwt(user);

                            let jwt = JWT{access_token};

                            Ok(Json(jwt))
                        },
                        None => {
                            return Err(Unauthorized(Some("User not found".to_string())));
                        }
                    }
                },
                None => {
                    return Err(Unauthorized(Some("Session not found".to_string())));
                }
            }
        },
        None => {
            return Err(Unauthorized(Some("Session not found".to_string())));
        }
    }
}

#[get("/logout")]
pub async fn logout(db: Db, cookies: &CookieJar<'_>) -> Result<Json<String>, Unauthorized<String>> {
    let session_token = cookies.get("session");

    match session_token {
        Some(session_tkn) => {
            let session_value = session_tkn.value().to_string();

            db.run(move |conn| {
                diesel::delete(sessions::table)
                .filter(sessions::token.eq(session_value))
                .execute(conn)
            }).await.ok();

            cookies.remove(Cookie::named("session"));

            return Ok(Json("Session deleted".to_string()));
        },
        None => {
            return Err(Unauthorized(Some("Session not found".to_string())));
        }
    }
}