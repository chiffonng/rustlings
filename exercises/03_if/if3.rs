// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    /* 
    This function should return where the animal lives. 
    
    If the animal is a "crab" -> id = 1 -> return "Beach"
    If the animal is a "gopher" -> id = 2 -> return "Burrow"
    If the animal is a "snake" -> id = 3 -> return "Desert"
    If the animal is anything else, return "Unknown".
    */
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2 // change from floating point to integer, to match below if
    } else if animal == "snake" {
        3
    } else {
        0 // change from string to integer
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
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
