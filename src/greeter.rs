// src/greeter.rs
pub fn greet(name: &str, age: u32) -> String {
    if age >= 18 {
        format!("Ndewo {}! Ị nwere ike ịtụ vootu. (Igbo)", name)
    } else if age >= 13 {
        format!("E kaaro {}! O tun le dibo. (Yoruba)", name)
    } else {
        format!("Sannu {}! Ba za ka iya jefa kuri'a ba. (Hausa)", name)
    }
}