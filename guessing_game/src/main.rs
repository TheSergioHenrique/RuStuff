//PRELUDE
use std::io;//io comes from the standard library.

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess!");

    let mut guess = String::new(); //variable are imutable by default. So, we add the mut
                                   //sufix. String::new generates a new String instance directly
                                   //from STDlib.

    io::stdin()
        .read_line(&mut guess) //searches what is in the adress of mut guess at the moment.
        .expect("Failed to read line"); //readline is expected to send back a result value. Result is an Enum, which can be in multiple states, so to make the code safer, we need to put the .expect in the end of a call.

    println!("You guessed: {guess}");
}

