use std::io;
use crate::data::{Exercise, MuscularGroup};

pub fn start_msg() {
    println!("Welcome to the gym-cli! \nLets start! \nFirst i need to know, what training split will we do (PPL or UP/LW): ");
}

pub fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input!");
    input.trim().to_string()
}

pub fn read_number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{prompt}");
        let input = user_input();
        match input.parse::<T>() {
            Ok(valor) => return valor,
            Err(_) => println!("Invalid value, try again."),
        }
    }
}

pub fn read_muscular_group() -> MuscularGroup {
    loop {
        println!("Choose the muscular group: {}", MuscularGroup::menu());
        let index: u8 = read_number("Enter the number: ");
        match MuscularGroup::from_index(index) {
            Some(mg) => return mg,
            None => println!("Invalid option, try again."),
        }
    }
}

pub fn read_exercise() -> Exercise {
    println!("Exercise name:");
    let name = user_input();

    let muscular_group = read_muscular_group();
    let series: u8 = read_number("How many sets?");
    let reps: u8 = read_number("How many reps?");
    let weight_kg: f32 = read_number("What weight (kg)?");

    Exercise {
        name,
        muscular_group,
        series,
        reps,
        weight_kg,
    }
}