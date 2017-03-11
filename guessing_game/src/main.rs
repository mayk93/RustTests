// This is an external dependency. We specified this dependency in the toml file.
extern crate rand;

// For convenience mostly. We use stdin from io. By doing this here, we don'y have to do this later:
// str::io::stdin
use std::io;
// Rng is "imported" from the rand crate. While we don't use it directly, we need it to use another
// method. See "traits". ( Sort of like an interface, not really )
use rand::Rng;

fn main() {
    println!("Guess the number!\nPlease input your guess.");

    // guess is more or less a variable (binding)
    // While by default it would be immutable, we say mut, so it is mutable.
    // String is a type we have access to from the stdlib.
    // String has a "associated method" (static method) called "new". That is why we call with ::
    // This is how we tell the compiler guess is a mutable, growable utf-9 encoded array
    let mut guess = String::new();

    // secret_number is immutable. From the rand
    // We need to "import" Rng at first in order to use the associated method thread_rng and the
    // method gen_range.
    let secret_number = rand::thread_rng().gen_range(0, 101);

    // io::stdin() from io we call the "associated method" (static methid) stdin to instantiate
    // a instance of the standard input.
    // The standard input has a method called read_line. This is why we call it with
    // . as opposed to ::
    io::stdin()
        .read_line(&mut guess) // We are passing a reference to the mutable. We are putting guess
                               // in and we expect it to be changed by the read_line method.
        .expect("Failed to read line"); // read_line returns a Result type, which has a expect()
                                        // method.

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);
}