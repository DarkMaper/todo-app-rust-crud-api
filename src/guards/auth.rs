pub mod authentication_guard {

    pub use rocket::request::{self, Request, FromRequest};
    use rocket::http::Status;
    use rocket::serde::json::{json, Value};

    pub use jsonwebtoken::{decode, Validation, DecodingKey, TokenData, Algorithm, errors::{ErrorKind, Error}};

    use crate::controllers::user_controllers::Claims;

    pub struct Token(pub TokenData<Claims>);

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Token {
        type Error = Value;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            let authorization_header = request.headers().get_one("authorization");

            match authorization_header {
                Some(data) => {
                    let has_bearer = data.contains("Bearer");
                    if !has_bearer {
                        return request::Outcome::Failure((Status::BadRequest, json!({ "error": "Missing Bearer" })));
                    };

                    let header_split:Vec<&str> = data.split_whitespace().collect();

                    let token = decode::<Claims>(&header_split[1], &DecodingKey::from_secret(b"secret"), &Validation::default());


                    match token {
                        Ok(token) => {
                            request::Outcome::Success(Token(token))
                        },
                        Err(error) => {
                            match error.kind() {
                                ErrorKind::ExpiredSignature => {
                                    return request::Outcome::Failure((Status::Unauthorized, json!({ "message": "jwt expired" })));
                                },
                                _ => {
                                    return request::Outcome::Failure((Status::Unauthorized, json!({ "message": "Invalid Token" })));
                                }
                            }
                        },
                    }
                }
                None => request::Outcome::Failure((Status::Unauthorized, json!({ "message": "Missing Token" })))
            }
        }
    }
}