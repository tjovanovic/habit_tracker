use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde_json::Value;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

use crate::core::habits::Habit;

// we can extract the connection pool with `State`
pub async fn get(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}


struct HabitRequest {

}

pub async fn post(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> Result<String, (StatusCode, String)> {
    println!("Payload: {}", payload);
    let h: Habit = serde_json::from_value(payload).map_err(internal_error)?;

    // println!("Habit: {}", h);

    sqlx::query("insert into habits (name) values ($1)")
        .bind(result.name)
        .execute(&pool)
        .await
        .map(|_s| String::from("Ok"))
        .map_err(internal_error)

    // Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Test")))

    Ok(String::from("Accepted"))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
