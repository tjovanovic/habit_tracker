use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use sqlx::postgres::PgPool;
use std::collections::HashMap;

use crate::core::habits::{Habit, HabitType, WeekDay};

// we can extract the connection pool with `State`
pub async fn get(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

pub async fn post(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
    body: String,
) -> Result<String, (StatusCode, String)> {
    // let result = Habit::new(
    //     String::from("Test"),
    //     vec![WeekDay::Monday, WeekDay::Tuesday],
    //     HabitType::O,
    // );
    // println!("{}", body);
    // for (key, val) in params.into_iter() {
    //     println!("{} / {}", key, val);
    // }
    // sqlx::query("insert into habits (name) values ($1)")
    //     .bind(result.name)
    //     .execute(&pool)
    //     .await
    //     .map(|_s| String::from("Ok"))
    //     .map_err(internal_error)

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
