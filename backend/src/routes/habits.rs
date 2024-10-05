use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json::Value;

use crate::app::{App, MyError};
use crate::core::habits::{Habit, HabitId, HabitType, Priority, WeekDay};
use crate::core::users::UserId;

#[derive(Deserialize, Debug)]
pub struct HabitGetRequest {
    pub id: HabitId,
}

pub async fn get(State(app): State<App>, Json(payload): Json<Value>) -> Result<String, MyError> {
    let request: HabitGetRequest = serde_json::from_value(payload)?;
    let habit = app.get_habit(request.id).await?;
    Ok(serde_json::to_string(&habit)?)
}

#[derive(Deserialize, Debug)]
pub struct HabitPostRequest {
    pub name: String,
    pub desired_week_days: Vec<WeekDay>,
    pub habit_type: HabitType,
    pub category: String,
    pub priority: Priority,
}

pub async fn post(State(state): State<App>, Json(payload): Json<Value>) -> Result<String, MyError> {
    println!("Payload: {}", payload);
    let request: HabitPostRequest = serde_json::from_value(payload)?;
    println!("Request: {:#?}", request);

    let habit = Habit::new(
        HabitId(2),
        request.name,
        Vec::new(),
        Vec::new(),
        request.habit_type,
        request.category,
        request.priority,
        UserId(1),
    );

    state.create_habit(habit).await.map_err(|err| err.into())
}
