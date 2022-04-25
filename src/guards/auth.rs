pub mod authentication_guard {

    pub use rocket::request::{self, Request, FromRequest};
    use rocket::http::Status;

    pub use jsonwebtoken::{decode, Validation, DecodingKey, TokenData};

    use crate::controllers::user_controllers::Claims;

    pub struct Token(pub TokenData<Claims>);

    #[derive(Debug)]
    pub enum ApiTokenError {
        Missing,
        Invalid,
    }


    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Token {
        type Error = ApiTokenError;

        async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            let authorization_header = request.headers().get_one("authorization");

            match authorization_header {
                Some(data) => {
                    let has_bearer = data.contains("Bearer");
                    if !has_bearer {
                        return request::Outcome::Failure((Status::BadRequest, ApiTokenError::Missing));
                    };

                    let header_split:Vec<&str> = data.split_whitespace().collect();

                    let token = decode::<Claims>(&header_split[1], &DecodingKey::from_secret(b"secret"), &Validation::default());

                    match token {
                        Ok(token) => {
                            request::Outcome::Success(Token(token))
                        },
                        Err(_) => request::Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
                    }
                }
                None => request::Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
            }
        }
    }
}