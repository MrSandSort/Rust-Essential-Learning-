// use std::io;
use rand;

fn main()
{
    /*
    let mut buffer= String::new();
    println!("Enter your message: ");
    io::stdin().read_line(&mut buffer);
    println!("The buffer message is: {}", buffer);

    // parsing needs to be done whenever we need to convert string to some other type
    // in rust we simply use unwrap method or can use turbofish operator

    let mut number= buffer.trim().parse::<i32>().unwrap();
    println!("The new number is: {}", number+1);
    */

    // using random number generator

    let number=rand::random::<f64>();
    println!("The random number is: {}", number);


}