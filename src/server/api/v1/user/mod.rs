use crate::{
    server::{
        htmx::{hx_redirect, IsHtmx},
        internal_error, internal_error_old,
        state::{user::User, App},
        UserSession,
    },
    EmailAddress,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

mod verify;

pub fn router() -> axum::Router<App> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/display_name", get(display_name))
}

#[derive(Debug, Deserialize)]
pub struct RegisterInfo {
    display_name: String,
    email_address: EmailAddress,
    password: String,
}

#[axum::debug_handler]
pub async fn register(
    user: State<User>,
    is_htmx: IsHtmx,
    form: Form<RegisterInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::register::Error;

    match user
        .register(&form.email_address, &form.password, &form.display_name)
        .await
    {
        Ok(_) if *is_htmx => (StatusCode::OK, [hx_redirect("/login")]).into_response(),
        Ok(_) => (StatusCode::OK, "your account has been registered").into_response(),

        Err(Error::DuplicateEmail) => {
            (StatusCode::CONFLICT, "email address already in use").into_response()
        }

        Err(Error::Database(error)) => internal_error(error).into_response(),
        Err(Error::PasswordHash(error)) => internal_error(error).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    email_address: EmailAddress,
    password: String,
}

#[allow(clippy::disallowed_types)]
#[axum::debug_handler]
async fn login(
    user: State<User>,
    session: tower_sessions::Session,
    is_htmx: IsHtmx,
    form: Form<LoginInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::login::Error;

    if !session.is_empty().await {
        return (StatusCode::CONFLICT, "you are already logged in").into_response();
    }

    match user.login(&form.email_address, &form.password).await {
        Ok(user_id) => {
            session.insert("id", user_id).await.unwrap();

            if *is_htmx {
                (StatusCode::OK, [hx_redirect("/")]).into_response()
            } else {
                (StatusCode::OK, "you have been logged in").into_response()
            }
        }

        Err(Error::InvalidLogin) => {
            (StatusCode::UNAUTHORIZED, "invalid login credentials").into_response()
        }

        Err(Error::Database(error)) => internal_error(error).into_response(),
        Err(Error::PasswordHash(error)) => internal_error(error).into_response(),
    }
}

async fn display_name(state: State<App>, user_session: UserSession) -> impl IntoResponse {
    let user_id = user_session.get_id().await;

    match query!(
        "
        SELECT display_name
            FROM _user.profile
            WHERE id = $1
        ;
        ",
        user_id
    )
    .fetch_one(state.db())
    .await
    {
        Ok(record) => (StatusCode::OK, record.display_name).into_response(),
        Err(error) => (internal_error_old(error), "Internal server error.").into_response(),
    }
}
