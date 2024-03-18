use std::env;
use anyhow::Context;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use controllable_pass::models::{Password, NewPassword};
use controllable_pass::repositories::password::{list_passwords, create_password};

use axum::{
    routing::{get,post},
    http::StatusCode,
    Json, Router,
    extract::State,
    response::{IntoResponse, Response},
};

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL")
        .with_context(|| format!("please set DATABASE_URL environment variable"))?;

    let password_routes = Router::new()
        .route("/", get(list_passwords_handler))
        .route("/", post(create_password_handler));
    let api_v0_routes = Router::new()
        .nest("/passwords", password_routes);
    let api_routes = Router::new().nest("/v0", api_v0_routes);

    let db_config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&db_url);
    let db_pool = bb8::Pool::builder().build(db_config).await?;

    // build our application with a route
    let app_routes = Router::new()
        .nest("/api", api_routes)
        .with_state(db_pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app_routes).await?;

    Ok(())
}

async fn create_password_handler(
    State(pool): State<Pool>,
    Json(payload): Json<NewPassword>,
) -> Result<Json<Password>, AppError> {
    let mut conn = pool.get().await?;
    let password = create_password(&mut conn, payload).await?;

    Ok(Json(password))
}

async fn list_passwords_handler(
    State(pool): State<Pool>,
) -> Result<Json<Vec<Password>>, AppError> {
    let mut conn = pool.get().await?;
    let passwords = list_passwords(&mut conn).await?;

    Ok(Json(passwords))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
