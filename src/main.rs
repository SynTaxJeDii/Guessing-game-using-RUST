// ! To take input from the user we use a standard library 
use std::io;


fn main(){

    println!("Guess the number");

    println!("Enter your age: ");

    let mut guess = String::new();


    //^ It will append in guess not overwrite the content if there's some

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your age is {}",guess);

}


    

