use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

use crate::core::habits::{Habit, HabitId, HabitType, Priority, WeekDay};
use crate::routes::state::App;

#[derive(Deserialize, Debug)]
pub struct HabitGetRequest {
    pub habit_id: HabitId,
}
// we can extract the connection pool with `State`
pub async fn get(
    State(app): State<App>,
    Json(payload): Json<Value>,
) -> Result<String, (StatusCode, String)> {
    let request: HabitGetRequest = serde_json::from_value(payload).map_err(internal_error)?;
    // app.get
    // sqlx::query_scalar("select 'hello world from pg'")
    //     .fetch_one(&state.pg_pool)
    //     .await
    //     .map_err(internal_error)
    Ok(String::from("Temporary"))
}

#[derive(Deserialize, Debug)]
pub struct HabitPostRequest {
    pub name: String,
    pub desired_week_days: Vec<WeekDay>,
    pub habit_type: HabitType,
    pub category: String,
    pub priority: Priority,
}

pub async fn post(
    State(state): State<App>,
    Json(payload): Json<Value>,
) -> Result<String, (StatusCode, String)> {
    println!("Payload: {}", payload);
    let request: HabitPostRequest = serde_json::from_value(payload).map_err(internal_error)?;
    println!("Request: {:#?}", request);

    // sqlx::query("insert into habits (name) values ($1)")
    //     .bind(result.name)
    //     .execute(&pool)
    //     .await
    //     .map(|_s| String::from("Ok"))
    //     .map_err(internal_error);

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
