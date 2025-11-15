// src/voter.rs
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_voter(name: &str, age: u32, can_vote: bool) {
    let status = if can_vote { "Eligible" } else { "Not Eligible" };
    let entry = format!("Name: {}, Age: {}, Status: {}\n", name, age, status);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("voters.txt")
        .expect("Cannot open voters.txt");

    file.write_all(entry.as_bytes()).expect("Write failed");
}