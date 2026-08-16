# Gym CLI

An interactive command-line interface (CLI) workout plan generator developed entirely in Rust. 

Gym CLI allows you to create your workout plan (such as PPL or Upper/Lower), register your exercises, sets, and weights, and automatically export everything to a formatted PDF file ready to take to the gym.

---

## Features

* **Workout Splits:** Native support for classic splits like PPL (Push/Pull/Legs) and UP/LW (Upper/Lower).
* **Detailed Registration:** Log the exercise name, target muscle group, number of sets, repetitions, and weight (kg).
* **Terminal Summary:** View your complete formatted workout plan directly in the terminal before saving.
* **PDF Export:** Generates a clean `workout_plan.pdf` file, organized by training days and including the creation date.

---

## Technologies Used

* **[Rust](https://www.rust-lang.org/)**: Main language used for safety and performance.
* **[printpdf](https://crates.io/crates/printpdf)**: For structured PDF file generation.
* **[chrono](https://crates.io/crates/chrono)**: For date and time manipulation in the PDF footer.

---

## How to Run

### Prerequisites
You will need to have [Rust and Cargo](https://rustup.rs/) installed on your machine.

### Standard Installation
1. Clone this repository:
   ```bash
   git clone [https://github.com/](https://github.com/)[YOUR_USERNAME]/gym-cli.git
   cd gym-cli
