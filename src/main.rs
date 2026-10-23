use rand::RngExt;
use std::io;

fn main() 
{
    /*
    let a= 20;
    let b=45.0;

    let sum= a as f64+b ;
    let diff= b-a as f64;
    let prod= a as f64 *b ;
    let division= b /a as f64;
    let remainder= b%a as f64;

    println!("The sum of two num is: {}", sum);
    println!("The difference of two num is: {}", diff);
    println!("The product of two num is: {}", prod);
    println!("The division of two num is: {}", division);
    println!("The remainder of two num is: {}", remainder);  
    
   

    let number=rand::random::<f64>();
    println!("The random number is: {}", number);

    let number= rand::rng().random_range(1..11); // current package
    println!("The number is: {}", number);

     */


     let random= rand::rng().random_range(1..100);
     let mut steps=0;
     let mut guessed= false;
        
     while !guessed
     {
        let mut input= String::new();
     
        println!("Guess the number:");
        io::stdin().read_line(&mut input);
     
        let number: u32= input.trim().parse().unwrap();
        
        if number > 100 || number < 1   
        {
            println!("The number needs to be valid between 0 and 100");
            continue;
        }

        if number > random
        {
            println!("You need to guess lower.");
            steps +=1; 
        }

        else if number < random
        {
            println!("You need to guess higher.");
            steps +=1; 
        }
        else
        {
            println!("Your guess is correct!");
            println!("The number is: {}", random);
            println!("The steps taken is: {}", steps);
            guessed= true;
        }
     }


     println!("The random number is: {}", random);


} 