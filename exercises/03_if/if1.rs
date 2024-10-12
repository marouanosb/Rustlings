fn bigger(a: i32, b: i32) -> i32 {
<<<<<<< HEAD
    if a > b {
        a
    }
    else {
        b
    }
}

fn main() {
    let x = 5;
    let y = 10;
    println!("{} is bigger.",bigger(x, y));
=======
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
}

fn main() {
    // You can optionally experiment here.
>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
