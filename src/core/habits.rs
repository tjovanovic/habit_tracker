use std::vec::Vec;

#[derive(PartialEq, Debug)]
struct Habit {
    name: String,
    desired_week_days: Vec<WeekDay>,
    completed_week_days: Vec<WeekDay>,
    habit_type: HabitType,
}

#[derive(PartialEq, Debug)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(PartialEq, Debug)]
enum HabitType {
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

    pub fn complete_day(&mut self, day: WeekDay) {
        if !self.completed_week_days.contains(&day) {
            self.completed_week_days.push(day);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
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
}
