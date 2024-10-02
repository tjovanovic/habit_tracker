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

// impl<'r> sqlx::FromRow<'r, PgRow> for Habit {
//     fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//         let author_id = row.try_get("author_id")?;
//         let name = row.try_get("name")?;
//         let dob = row.try_get("dob")?;
//         // let books: Json<_> = row.try_get("books")?;
//         Ok(Author {
//             author_id,
//             name,
//             dob,
//             books: books.0,
//         })
//     }
// }

pub fn from_row(row: PgRow) -> Result<Habit, sqlx::Error> {
    let id: i32 = row.try_get("id")?;
    let id = HabitId(id);
    let name = row.try_get("name")?;
    // let desired_week_days = row.try_get("desired_week_days")?;
    // let books: Json<_> = row.try_get("books")?;
    Ok(Habit::new(
        id,
        name,
        Vec::new(),
        // desired_week_days,
        None,
        HabitType::Length(1, 2, 3),
        String::from("lel"),
        habits::Priority::P1,
        UserId(43),
    ))
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

        // let result = match result {
        //     Err(_e) => {
        //         return None;
        //     }
        //     Ok(row) => row,
        // };

        // let result: Habit = result.try_into()?;

        // .map(|_s| String::from("Ok"))
        // .map_err(internal_error);
        // None
    }

    pub fn create_habit(&self, req: HabitPostRequest) -> () {}
}
