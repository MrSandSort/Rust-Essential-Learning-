fn main()
{
    say_hello();
    let square_val= square(12);
    println!("The square of this num is: {}", square_val);
}

fn say_hello()
{
    println!("Hello, Namaste Nepal");
    say_a_number(24);
    find_sum(20,30);
    
}

fn say_a_number(number: i32)
{
    println!("The number is: {}", number);
}

fn find_sum(first_num: i8, second_num: i8)
{
    let sum: i8= first_num + second_num;
    println!("The sum of two numbers are: {}", sum);
}

fn square(num: i32) ->i32
{
    println!("The number used for square is: {}", num);
    return num * num;
}