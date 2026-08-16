mod data;
mod func;
mod pdf;
mod validation;

use crate::{
    func::{start_msg, user_input, read_number, read_exercise},
    validation::validate_input,
    data::{split_to_workouts, Workout},
    pdf::generate_pdf,
};

fn main() {
    start_msg();
    let training_split = user_input();

    let split = match validate_input(&training_split) {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let workout_types = match split_to_workouts(&split) {
        Ok(types) => types,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let mut workouts: Vec<Workout> = workout_types
        .into_iter()
        .map(|wt| Workout {
            workout_type: wt,
            exercises: vec![],
        })
        .collect();

    for workout in workouts.iter_mut() {
        println!("\n=== Day {} ===", workout.workout_type);
        let quantidade: u8 = read_number("How many exercises for this day?");

        for _ in 0..quantidade {
            let exercise = read_exercise();
            workout.exercises.push(exercise);
        }
    }

    println!("\n\n===== YOUR WORKOUT PLAN =====");
    for workout in &workouts {
        println!("\n-- {} --", workout.workout_type);
        for ex in &workout.exercises {
            println!(
                "{} | {} | {}x{} | {}kg",
                ex.name, ex.muscular_group, ex.series, ex.reps, ex.weight_kg
            );
        }
    }

    println!("\n\nDo you want to save this workout plan as PDF? (yes/no): ");
    let answer = user_input();
    
    if answer.to_lowercase() == "yes" || answer.to_lowercase() == "y" {
        let filename = "workout_plan.pdf";
        match generate_pdf(&workouts, filename) {
            Ok(_) => {
                println!("\n PDF saved successfully as '{}'!", filename);
                println!(" You can find it in the project folder.");
            }
            Err(e) => {
                println!("\n Error generating PDF: {}", e);
            }
        }
    } else {
        println!("\n Okay, no PDF generated. Have a great workout!");
    }
}
