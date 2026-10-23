use std::env;

fn main()
{
    if env::args().len() <= 2
    {
       println!("We need to have atleast 2 arguments!"); 
       return;
    }
    for (index, argument) in env::args().enumerate()
    {
        println!("Argument {} is {}", index, argument);
    } 

    let arg2= env::args().nth(2).unwrap();
    let number:i32= arg2.trim().parse().unwrap();

    println!("The second argument is: {}", arg2);
}