use std::vec::Vec;

#[derive(PartialEq, Debug)]
pub struct Habit {
    pub name: String,
    pub desired_week_days: Vec<WeekDay>,
    completed_week_days: Vec<WeekDay>,
    pub habit_type: HabitType,
}

#[derive(PartialEq, Debug)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(PartialEq, Debug)]
pub enum HabitType {
    Completed,
    Length(u16),
}

impl Habit {
    pub fn new(name: String, desired_week_days: Vec<WeekDay>, habit_type: HabitType) -> Self {
        Habit {
            name,
            desired_week_days,
            completed_week_days: Vec::new(),
            habit_type,
        }
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
    use super::*;

    #[test]
    fn test_init() {
        let result = Habit::new(
            String::from("Test"),
            vec![WeekDay::Monday, WeekDay::Tuesday],
            HabitType::Completed,
        );
        let expected_habit = Habit {
            name: String::from("Test"),
            desired_week_days: vec![WeekDay::Monday, WeekDay::Tuesday],
            completed_week_days: Vec::new(),
            habit_type: HabitType::Completed,
        };
        assert_eq!(result, expected_habit);
    }

    #[test]
    fn test_completing() {
        let mut result = Habit::new(
            String::from("Test"),
            vec![WeekDay::Monday, WeekDay::Tuesday],
            HabitType::Completed,
        );
        result.complete_day(WeekDay::Monday);
        result.complete_day(WeekDay::Tuesday);
        assert_eq!(result.is_complete(), true);
    }
}
