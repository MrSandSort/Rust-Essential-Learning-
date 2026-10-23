use std::io;
use rand::RngExt;

fn main()
{
   let random_val: i32= rand::rng().random_range(1..7);
   roll_dice(&random_val);
}

fn roll_dice(random_val: &i32)
{
    let mut steps=0;
    loop
    {
        let mut user_input= String::new();
        println!("Enter the dice number here!");
        io::stdin().read_line(&mut user_input);
        let number:i32= user_input.trim().parse().unwrap();

        if steps >= 0 && steps <= 2 
        {
        if number < 1 || number > 6
        {
            println!("Enter the valid numbers");
            continue;
        } 

        if number > *random_val
        {
            println!("The dice number you guessed is higher.");
            steps += 1;
        }
        else if number < *random_val
        {
            println!("The dice number you guessed is lower.");
            steps += 1;
        }

        else
        {
            println!("The dice number you guessed is correct.");
            break;
        }
    }
    else
    {
        println!("You have reached your attempted steps.");
        println!("Try again bravo!!");
        println!("The number is: {}", *random_val);
        break;
    }

    }

}