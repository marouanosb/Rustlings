fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
<<<<<<< HEAD
        2
    } else if animal == "snake" {
        3
    } else {
        4
=======
        2.0
    } else if animal == "snake" {
        3
    } else {
        "Unknown"
>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
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
