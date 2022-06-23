use rocket::request::{FromRequest, Outcome};
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    /// timestamp
    // pub exp: i64,
    /// user id
    pub uid: i64,
    // pub username: String,
}

// #[derive(Debug)]
// pub struct AuthenticatedUser(i64);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, Self::Error> {
        request.cookies()
            .get_private("uid")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|val| AuthenticatedUser { uid: val })
            .or_forward(())
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for AuthenticatedUser {
//     type Error = LoginError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, LoginError> {
//         let email = request.headers().get_one("email");
//         let password = request.headers().get_one("password");

//         match (email, password) {
//             (Some(e), Some(p)) => {
//                 let user_auth = get_user_auth_by_email(e.to_string());

//                 match user_auth {
//                     Some(auth_info) => {
//                         let hash = hash(&String::from(p));
//                         if hash == auth_info.password_hash {
//                             Outcome::Success(AuthenticatedUser{uid: auth_info.uid})
//                         }
//                         else {
//                             Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
//                         }
//                     }
//                     None => Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist))
//                 }
//             },
//             _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
//         }
//     }
// }