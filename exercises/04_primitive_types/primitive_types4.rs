fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
<<<<<<< HEAD
        let nice_slice = &a[1..4];
=======

>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f
        assert_eq!([2, 3, 4], nice_slice);
    }
}
