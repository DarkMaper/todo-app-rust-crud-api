use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::serde::json::Json;
use bcrypt::{hash, verify};

use chrono::prelude::*;

use jsonwebtoken::{encode, Header, EncodingKey};

use crate::diesel::prelude::*;

type Result<T,E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use crate::models::user::*;
use crate::db::Db;

async fn check_user_exists(db: &Db, email: String) -> Result<Json<User>,bool> {

    let user = db.run(move |conn| {
        users::table
            .filter(users::email.eq(email.to_owned()))
            .first(conn)
    }).await.map(Json);

    match user {
        Ok(usr) => {
            Ok(usr)
        },
        Err(_) => {
            Err(false)
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
pub async fn signup(db: Db, signin: Json<User>) -> Result<Created<Json<String>>, (Status, String)> {

    let user_value = signin.clone();

    let check_user = check_user_exists(&db, user_value.email.clone()).await;

    match check_user {
        Ok(_) => {
            return Err((Status::Conflict, "Ya existe un usuario con ese email".to_string()));
        },
        Err(_) => {
            let hashed_password = hash(&user_value.password, 12).unwrap();
            let new_user = User {
                email: user_value.email.clone(),
                password: hashed_password
            };
                
            match db.run(move |conn| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .execute(conn)
            }).await
            {
                Ok(_) => return Ok(Created::new("/").body(Json("{ message: 'User created'}".to_string()))),
                Err(msg) => return Err((Status::new(200), msg.to_string()))
            }
        }
    };
}

#[post("/signin", data = "<login_form>")]
pub async fn signin(db: Db, login_form: Json<User>) -> Result<Json<JWT>, (Status, String)> {
    let user_value = login_form.clone();

    let check_user = check_user_exists(&db, user_value.email.clone()).await;
    
    match check_user { 
        Ok(user) => {
            let password_match = verify(user_value.password, &user.password);

            match password_match {
                Ok(result) => {
                    if !result { return Err((Status::Unauthorized, "El usuario o la contraseña no son correctos".to_string())); }
                    let expiration = Utc::now()
                        .checked_add_signed(chrono::Duration::minutes(15))
                        .expect("valid timestamp")
                        .timestamp();

                    let claims = Claims {
                        sub: user_value.email.to_string(),
                        exp: expiration as usize
                    };

                    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap();

                    let jwt = JWT{token};

                    Ok(Json(jwt))
                },
                Err(_) => {
                    return Err((Status::Unauthorized, "El usuario o la contraseña no son correctos".to_string()));
                }
            }
        },
        Err(_) => {
            return Err((Status::Unauthorized, "El usuario o la contraseña no son correctos".to_string()));
        }
    }

}