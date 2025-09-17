// ! To take input from the user we use a standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number");

    let secrete_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your number: ");

        let mut guess = String::new();

        //^ It will append in guess not overwrite the content if there's some

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //^ If we run it directly without using trim then it will not be able to parse it into integer because when we input our number we hit "Enter" and our input is (Suppose I've given 45 as input) 45\n. That's why we get new line after giving input

        //& Here we are shadowing the guess variable and the older guess variable is no more. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid input (Integer)");
                continue;
            },
        };

        //^ The guess used below is the shadowed one which is of type u32 not String

        // * Match is exhaustive -  It means that you can't only use the one possible scnario you've to use all possible scnario . In our case you can't only use the Equal case.

        match guess.cmp(&secrete_number) {
            Ordering::Equal => {
                println!("You won");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
        }
    }
}
