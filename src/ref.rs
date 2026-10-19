fn main()
{
    let mut number = 10;

     println!("The original number is: {}", number);

    change_number(&mut number);

    println!("The number is changed to: {}", number);
}

fn change_number(num: &mut i32)
{
    *num=20;
}




