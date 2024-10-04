use std::borrow::Borrow;
use std::io;

use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{FromRow, Pool, Postgres, Row};

use super::habits::HabitPostRequest;
use crate::core::habits::{self, Habit, HabitId, HabitType};
use crate::core::users::UserId;

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

    pub async fn create_habit(&self, habit: Habit) -> Result<String, sqlx::Error> {
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
}
