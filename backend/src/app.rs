use sqlx::{Pool, Postgres, Row};

use crate::{
    core::habits::{Habit, HabitId},
    core::users::UserId,
    routes::habits::HabitPostRequest,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Clone)]
pub struct App {
    pg_pool: Pool<Postgres>,
}

impl App {
    pub fn new(pg_pool: Pool<Postgres>) -> Self {
        App { pg_pool }
    }

    pub async fn get_habit(&self, id: HabitId) -> Result<Habit, sqlx::Error> {
        let HabitId(id) = id;
        let result = sqlx::query_as::<Postgres, Habit>("select * from habits where id = $1")
            .bind(id)
            .fetch_one(&self.pg_pool)
            .await;

        result
    }

    pub async fn create_habit(&self, request: HabitPostRequest) -> Result<String, sqlx::Error> {
        let next_habit_id = self.get_next_habit_id().await?;
        let user_id = self.get_user_id().await?;

        let habit = Habit::new(
            next_habit_id,
            request.name,
            Vec::new(),
            Vec::new(),
            request.habit_type,
            request.category,
            request.priority,
            user_id,
        );

        sqlx::query(
            "
            insert into habits (
                id,
                name,
                desired_week_days,
                completed_week_days,
                habit_type,
                category,
                priority,
                user_id
            ) values ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(habit.id)
        .bind(habit.name)
        .bind(habit.desired_week_days)
        .bind(habit.completed_week_days)
        .bind(habit.habit_type)
        .bind(habit.category)
        .bind(habit.priority)
        .bind(habit.user_id)
        .execute(&self.pg_pool)
        .await
        .map(|_| String::from("Ok"))
    }

    async fn get_next_habit_id(&self) -> Result<HabitId, sqlx::Error> {
        let row =
            sqlx::query("select nextval(pg_get_serial_sequence('habits', 'id'))::int4 as new_id")
                .fetch_one(&self.pg_pool)
                .await?;

        let id = row.try_get("new_id")?;
        Ok(HabitId(id))
    }

    async fn get_user_id(&self) -> Result<UserId, sqlx::Error> {
        Ok(UserId(1))
    }
}

pub struct MyError {
    status_code: StatusCode,
    message: String,
}

impl From<sqlx::Error> for MyError {
    fn from(err: sqlx::Error) -> Self {
        println!("Error: {}", err.to_string());
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        println!("Error: {}", err.to_string());
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        (self.status_code, self.message).into_response()
    }
}
