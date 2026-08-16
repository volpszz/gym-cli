use std::fmt;

#[derive(Debug)]
pub enum WorkoutType {
    Push,
    Pull,
    Legs,
    Upper,
    Lower,
}

impl fmt::Display for WorkoutType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nome = match self {
            WorkoutType::Push => "Push",
            WorkoutType::Pull => "Pull",
            WorkoutType::Legs => "Legs",
            WorkoutType::Upper => "Upper",
            WorkoutType::Lower => "Lower",
        };
        write!(f, "{nome}")
    }
}

#[derive(Debug)]
pub enum MuscularGroup {
    Chest,
    Back,
    Shoulders,
    Forearms,
    Biceps,
    Triceps,
    Quads,
    Hamstrings,
    Calf,
    Abs,
}

impl fmt::Display for MuscularGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nome = match self {
            MuscularGroup::Chest => "Chest",
            MuscularGroup::Back => "Back",
            MuscularGroup::Shoulders => "Shoulders",
            MuscularGroup::Forearms => "Forearms",
            MuscularGroup::Biceps => "Biceps",
            MuscularGroup::Triceps => "Triceps",
            MuscularGroup::Quads => "Quads",
            MuscularGroup::Hamstrings => "Hamstrings",
            MuscularGroup::Calf => "Calf",
            MuscularGroup::Abs => "Abs",
        };
        write!(f, "{nome}")
    }
}

impl MuscularGroup {
    pub fn from_index(index: u8) -> Option<MuscularGroup> {
        match index {
            1 => Some(MuscularGroup::Chest),
            2 => Some(MuscularGroup::Back),
            3 => Some(MuscularGroup::Shoulders),
            4 => Some(MuscularGroup::Forearms),
            5 => Some(MuscularGroup::Biceps),
            6 => Some(MuscularGroup::Triceps),
            7 => Some(MuscularGroup::Quads),
            8 => Some(MuscularGroup::Hamstrings),
            9 => Some(MuscularGroup::Calf),
            10 => Some(MuscularGroup::Abs),
            _ => None,
        }
    }

    pub fn menu() -> &'static str {
        "1-Chest 2-Back 3-Shoulders 4-Forearms 5-Biceps 6-Triceps 7-Quads 8-Hamstrings 9-Calf 10-Abs"
    }
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub muscular_group: MuscularGroup,
    pub series: u8,
    pub reps: u8,
    pub weight_kg: f32,
}

#[derive(Debug)]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub exercises: Vec<Exercise>,
}

pub fn split_to_workouts(split: &str) -> Result<Vec<WorkoutType>, String> {
    match split {
        "PPL" => Ok(vec![WorkoutType::Push, WorkoutType::Pull, WorkoutType::Legs]),
        "UP/LW" => Ok(vec![WorkoutType::Upper, WorkoutType::Lower]),
        _ => Err("Unknown split".to_string()),
    }
}