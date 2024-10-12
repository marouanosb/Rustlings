fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
<<<<<<< HEAD
        let second = numbers.1;
=======

>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
