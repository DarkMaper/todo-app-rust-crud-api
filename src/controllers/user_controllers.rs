use rocket::response::{Debug, status::{Created, Conflict, Unauthorized}};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use bcrypt::{hash, verify, DEFAULT_COST};

use chrono::prelude::*;

use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;

use crate::diesel::prelude::*;

type Result<T,E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use crate::models::user::*;
use crate::db::Db;

async fn check_user_exists(db: &Db, email: String) -> Option<Json<User>> {

    let user = db.run(move |conn| {
        users::table
            .filter(users::email.eq(email.to_owned()))
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
    pub token: String
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub exp: usize,
    pub sub: String
}

#[post("/signup", data = "<signin>")]
pub async fn signup(db: Db, signin: Json<User>) -> Result<Created<String>, Conflict<String>> {

    let user_value = signin.clone();

    let check_user = check_user_exists(&db, user_value.email.clone()).await;

    match check_user {
        Some(_) => {
            return Err(Conflict(Some("Email already in use".to_string())));
        },
        None => {
            let hashed_password = hash(&user_value.password, DEFAULT_COST).unwrap();
            let new_user = User {
                id: Some(Uuid::new_v4().to_string()),
                email: user_value.email.clone(),
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
pub async fn signin(db: Db, login_form: Json<User>) -> Result<Json<JWT>, Unauthorized<String>> {
    let user_value = login_form.clone();

    let check_user = check_user_exists(&db, user_value.email.clone()).await;
    
    match check_user { 
        Some(user) => {
            let password_match = verify(user_value.password, &user.password);

            match password_match {
                Ok(result) => {
                    if !result { 
                        return Err(Unauthorized(Some("Email or password are incorrect".to_string())));
                    }
                    let expiration = Utc::now()
                        .checked_add_signed(chrono::Duration::minutes(15))
                        .expect("valid timestamp")
                        .timestamp();

                    let claims = Claims {
                        sub: user.id.clone().unwrap(),
                        exp: expiration as usize
                    };

                    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap();

                    let jwt = JWT{token};

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