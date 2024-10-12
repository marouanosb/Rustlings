// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉
    // let your_character = '';
<<<<<<< HEAD
    let your_character = '1';
=======
>>>>>>> 5f23f738de0a4a2d7d833fd9c870f8b8d71a434f

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
