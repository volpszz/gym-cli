pub fn validate_input(input: &str) -> Result<String, String> {
    let trimmed = input.trim().to_uppercase();
    match trimmed.as_str() {
        "PPL" | "UP/LW" => Ok(trimmed),
        _ => Err("Invalid split! Please choose PPL or UP/LW.".to_string()),
    }
}
