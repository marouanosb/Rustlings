// TODO: Fix the compiler error on this function.
fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else {
<<<<<<< HEAD
        "bar"
=======
        1
>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f
    }
}

fn main() {
    // You can optionally experiment here.
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        // This means that calling `foo_if_fizz` with the argument "fizz" should return "foo".
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}
