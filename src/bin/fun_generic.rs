fn get_biggest <T:PartialOrd>(a: T, b: T)-> T
{
    if a > b
    {
        return a;
    }
    else
    {
        return b;
    }
}

fn main()
{
    println!("The biggest one is:  {}", get_biggest(10,20));
}