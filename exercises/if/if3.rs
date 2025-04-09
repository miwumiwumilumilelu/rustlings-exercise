// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


pub fn animal_habitat(animal: &str) -> &'static str {
    match animal {
        "crab" => "Beach",
        "gopher" => "Burrow",
        "snake" => "Desert",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
