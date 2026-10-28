fn longest_name <'a>(first: &'a str, second: &'a str)-> &'a str
{
    if first.len() > second.len()
    {
        return first;
    }
    else
    {
        return second;
    }
}


fn main()
{
    let first_name= String::from("Sandesh");
    let second_name= String::from("Sagar");
    println!("The longest string is {}", longest_name(&first_name, &second_name));
}