// This is an external dependency. We specified this dependency in the toml file.
extern crate rand;

// For convenience mostly. We use stdin from io. By doing this here, we don'y have to do this later:
// str::io::stdin
use std::io;
// We "import" this type in order to use it in a comparison.
use std::cmp::Ordering;

// Rng is "imported" from the rand crate. While we don't use it directly, we need it to use another
// method. See "traits". ( Sort of like an interface, not really )
use rand::Rng;

fn continue_loop(fails: &mut i32) {
    if *fails > 5 {
       println!("You have failed {} times. Maybe you should quit?", fails);
    }
    *fails += 1
}

fn main() {
    println!("Guess the number!\nPlease input your guess.");

    // secret_number is immutable. From the rand
    // We need to "import" Rng at first in order to use the associated method thread_rng and the
    // method gen_range.
    let secret_number = rand::thread_rng().gen_range(0, 101);

    // This number counts the number of failed guess attempts. After a number of iterations,
    // we start prompting the user if he wants to give up.
    let mut failed = 0;

    loop {
        // guess is more or less a variable (binding)
        // While by default it would be immutable, we say mut, so it is mutable.
        // String is a type we have access to from the stdlib.
        // String has a "associated method" (static method) called "new". That is why we call with ::
        // This is how we tell the compiler guess is a mutable, growable utf-9 encoded array
        let mut guess = String::new();

        // io::stdin() from io we call the "associated method" (static methid) stdin to instantiate
        // a instance of the standard input.
        // The standard input has a method called read_line. This is why we call it with
        // . as opposed to ::
        io::stdin()
            .read_line(&mut guess) // We are passing a reference to the mutable. We are putting guess
                                   // in and we expect it to be changed by the read_line method.
            .expect("Failed to read line"); // read_line returns a Result type, which has a expect()
                                            // method.

        // Here, we convert the String type guess to a int type guess.
        // This is called shadowing. We have already declared "guess" as a String, but we are creating
        // a new one that has type i32. This is, to be able to compare it with the secret_number.

        let guess: i32 = match guess.trim().parse() {
            Ok(result) => { result },
            Err(_)     => {
                println!("An error has occurred. Most likely, you did not input a number.");
                continue
            }
        };

        println!("You guessed: {}", guess);

        // We are calling the cmp (compare) method of the guess variable. This returns the Ordering
        // type we "imported" earlier.
        // Ordering is an enum. We are saying here that, in case Ordering is Less ( we are matching
        // that ), we execute println!("Too small!") and so on and so forth for the other 2 cases.
        match guess.cmp(&secret_number) {
            Ordering::Less    => {
                println!("Too small!");
                continue_loop(&mut failed);
            },
            Ordering::Greater => {
                println!("Too big!");
                continue_loop(&mut failed);
            },
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        }
    }

    println!("[] The secret number was: {}", secret_number);
}