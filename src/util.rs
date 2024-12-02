use std::fs;

pub fn get_input(day: u32) -> String {
    let path = format!("src/day_{}/in", day);
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn get_input_part(day: u32, part: u32) -> String {
    let path = format!("src/day_{}/in_{}", day, part);
    fs::read_to_string(path).expect("Failed to read file")
}
