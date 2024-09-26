//! Example of application using <https://github.com/launchbadge/sqlx>
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-sqlx-postgres
//! ```
//!
//! Test with curl:
//!
//! ```not_rust
//! curl 127.0.0.1:3000
//! curl -X POST 127.0.0.1:3000
//! ```

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod core;
pub mod routes;
use routes::root;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:example@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/api", get(root::get).post(root::post))
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
