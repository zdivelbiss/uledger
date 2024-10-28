use crate::server::{
    htmx::{hx_redirect, IsHtmx},
    internal_error,
    state::{user::account::UserAccount, App},
    UserSession,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use lib::EmailAddress;

mod verify;

pub fn router() -> axum::Router<App> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post({
            #[derive(Debug, Deserialize)]
            pub struct RegisterInfo {
                display_name: String,
                email_address: EmailAddress,
                password: String,
            }

|
            user_account: State<UserAccount>,
            is_htmx: IsHtmx,
            form: Form<RegisterInfo>,
 | async move {
            use crate::server::state::user::account::RegisterError;
        
            match user_account
                .register(&form.email_address, &form.password, &form.display_name)
                .await
            {
                Ok(_) if *is_htmx => (StatusCode::OK, [hx_redirect("/login")]).into_response(),
                Ok(_) => (StatusCode::OK, "your account has been registered").into_response(),
        
                Err(RegisterError::DuplicateEmail) => {
                    (StatusCode::CONFLICT, "email address already in use").into_response()
                }
        
                Err(RegisterError::Database(error)) => internal_error(error).into_response(),
                Err(RegisterError::PasswordHash(error)) => internal_error(error).into_response(),
            }
        }
        }))
        .route("/login", post(login))
        .route("/display_name", get(display_name))
}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    email_address: EmailAddress,
    password: String,
}

#[allow(clippy::disallowed_types)]
#[axum::debug_handler]
async fn login(
    user_account: State<UserAccount>,
    session: tower_sessions::Session,
    is_htmx: IsHtmx,
    form: Form<LoginInfo>,
) -> impl IntoResponse {
    use crate::server::state::user::account::LoginError;

    if !session.is_empty().await {
        return (StatusCode::CONFLICT, "you are already logged in").into_response();
    }

    match user_account
        .login(&form.email_address, &form.password)
        .await
    {
        Ok(user_id) => {
            session.insert("id", user_id).await.unwrap();

            if *is_htmx {
                (StatusCode::OK, [hx_redirect("/")]).into_response()
            } else {
                (StatusCode::OK, "you have been logged in").into_response()
            }
        }

        Err(LoginError::InvalidCredentials) => {
            (StatusCode::UNAUTHORIZED, "invalid login credentials").into_response()
        }

        Err(LoginError::Database(error)) => internal_error(error).into_response(),
        Err(LoginError::PasswordHash(error)) => internal_error(error).into_response(),
    }
}

async fn display_name(state: State<App>, user_session: UserSession) -> impl IntoResponse {
    let user_id = user_session.get_user_id().await;

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
        Err(error) => internal_error(error).into_response(),
    }
}
