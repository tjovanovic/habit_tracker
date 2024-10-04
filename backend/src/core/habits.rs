use super::users::UserId;
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize};
use serde_json::json;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgRow, PgTypeInfo, PgTypeKind, PgValueRef};
use sqlx::{Decode, Encode, Postgres};
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::vec::Vec;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Habit {
    pub id: HabitId,
    pub name: String,
    pub desired_week_days: Vec<WeekDay>,
    pub completed_week_days: Vec<WeekDay>,
    pub habit_type: HabitType,
    pub category: String,
    pub priority: Priority,
    pub user_id: UserId,
}

#[derive(Deserialize, Serialize, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct HabitId(pub i32);

#[derive(PartialEq, Deserialize, Serialize, Debug, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "varchar")]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize, Serialize, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "varchar")]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
    P5,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum HabitType {
    // Habit that can be either done or not
    OneOff,

    // Habit based on length
    Length(u16, u16, u16),
}

impl sqlx::Type<Postgres> for HabitType {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("json")
    }
}

impl<'r> Decode<'r, Postgres> for HabitType {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = value.as_str()?;
        let habit_type = serde_json::from_str::<HabitType>(value)?;
        Ok(habit_type)
    }
}

impl Encode<'_, Postgres> for HabitType {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let x = serde_json::to_writer(&mut **buf, self)?;
        Ok(IsNull::No)
    }
}

impl Habit {
    pub fn new(
        id: HabitId,
        name: String,
        desired_week_days: Vec<WeekDay>,
        completed_week_days: Vec<WeekDay>,
        habit_type: HabitType,
        category: String,
        priority: Priority,
        user_id: UserId,
    ) -> Self {
        Habit {
            id,
            name,
            desired_week_days,
            completed_week_days,
            habit_type,
            category,
            user_id,
            priority,
        }
    }

    pub fn times_per_week(&self) -> usize {
        return self.desired_week_days.len();
    }

    pub fn complete_day(&mut self, day: WeekDay) -> () {
        if !self.completed_week_days.contains(&day) {
            self.completed_week_days.push(day);
        }
    }

    pub fn is_complete(&self) -> bool {
        self.completed_week_days.len() <= self.desired_week_days.len()
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_init() {
    //     let result = Habit::new(
    //         String::from("Test"),
    //         vec![WeekDay::Monday, WeekDay::Tuesday],
    //         HabitType::Completed,
    //     );
    //     let expected_habit = Habit {
    //         name: String::from("Test"),
    //         desired_week_days: vec![WeekDay::Monday, WeekDay::Tuesday],
    //         completed_week_days: Vec::new(),
    //         habit_type: HabitType::Completed,
    //     };
    //     assert_eq!(result, expected_habit);
    // }

    // #[test]
    // fn test_completing() {
    //     let mut result = Habit::new(
    //         String::from("Test"),
    //         vec![WeekDay::Monday, WeekDay::Tuesday],
    //         HabitType::Completed,
    //     );
    //     result.complete_day(WeekDay::Monday);
    //     result.complete_day(WeekDay::Tuesday);
    //     assert_eq!(result.is_complete(), true);
    // }
}
