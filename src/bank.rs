fn main()
{
    let mut balance: f64= 2150.50;
    
    deposit(&mut balance, 2000.0);

    println!("The total balance is: {}", balance);
}

fn deposit(bal: &mut f64, amount: f64)
{
    *bal += amount;
}