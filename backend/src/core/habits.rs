use super::users::UserId;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct Habit {
    pub id: HabitId,
    pub name: String,
    pub desired_week_days: Vec<WeekDay>,
    completed_week_days: Vec<WeekDay>,
    pub habit_type: HabitType,
    pub category: String,
    priority: Priority,
    user_id: UserId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HabitId(pub i64);

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
    P5,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum HabitType {
    // Habit that can be either done or not
    OneOff,

    // Habit based on length
    Length(u16, u16, u16),
}

impl Habit {
    pub fn new(
        id: HabitId,
        name: String,
        desired_week_days: Vec<WeekDay>,
        completed_week_days: Option<Vec<WeekDay>>,
        habit_type: HabitType,
        category: String,
        priority: Priority,
        user_id: UserId,
    ) -> Self {
        Habit {
            id,
            name,
            desired_week_days,
            completed_week_days: match completed_week_days {
                None => Vec::new(),
                Some(x) => x,
            },
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
