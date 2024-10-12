fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
<<<<<<< HEAD
    let a = [..5];
=======
>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
