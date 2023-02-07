use axum::{
    http::{StatusCode, header},
    response::{IntoResponse},
    routing::{get, post}, Router,
    extract::{State, Json}
};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;


use uuid_rs::v4;

use crate::database::DB;

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub struct RouterState {
    dbconn: PgPool,
}

pub fn router(dbconn: PgPool) -> Router {

    let state = Arc::new(RouterState{dbconn});

    Router::new()
        .route("/", get(hello_world))
        .route("/register", post(Auth::register))
        .route("/login", post(Auth::login))
        .with_state(state)
}

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello, world!").into_response()
}

pub struct Auth {}

impl Auth {
    async fn register(State(state): State<Arc<RouterState>>, Json(login): Json<Login>) -> impl IntoResponse {

        let hashed_password = bcrypt::hash(login.password, 10).expect("Failed to hash password");

        let record = DB::create_user(login.email, hashed_password, state.dbconn.clone());
    
        match record.await {
            Ok(_) => {(StatusCode::CREATED, "Successfully created.").into_response()},
            Err(_) => {(StatusCode::INTERNAL_SERVER_ERROR, "There was an internal server error.").into_response()}
        }
    }

    async fn login(State(state): State<Arc<RouterState>>, Json(login): Json<Login>) -> impl IntoResponse {
        if let Ok(result) = DB::get_user(login.email, state.dbconn.clone()).await {

            match bcrypt::verify(login.password, &result.password) {
                Ok(_) => {(StatusCode::BAD_REQUEST, "Incorrect credentials. Please try again.").into_response()},
                Err(_) => {(StatusCode::BAD_REQUEST, "Incorrect credentials. Please try again.").into_response()},
            };

            let cookie = Auth::create_cookie();
            
            ((StatusCode::OK, "Logged in!".to_string()), (header::SET_COOKIE, cookie))
        } else {
            ((StatusCode::BAD_REQUEST, "Incorrect credentials. Please try again.".to_string()), (header::CONTENT_TYPE, "text/plain".to_string()))
        };
        
        
    }

    // async fn create_session(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, jar: SignedCookieJar) -> impl IntoResponse {
    //     if let Some(session_id) = Auth::authorize_and_create_session(auth.token()).await {
    //         Ok((
    //             // the updated jar must be returned for the changes
    //             // to be included in the response
    //             jar.add(Cookie::new("session_id", session_id)),
    //             Redirect::to("/me"),
    //         ))
    //     } else {
    //         Err(StatusCode::UNAUTHORIZED)
    //     }
    // }

    fn create_cookie() -> String {
        let cookie = format!("axum.sid={}; HttpOnly; Max-Age=3600", v4!());

        cookie
    }

    fn authorize_session() {}
}


