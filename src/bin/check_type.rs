use std::any;
use std::fmt;

// the check type functions uses the Debug or Display trait to be used
// we can also have multiple trait bounds by simply using adding + at the end of first, second or last


fn check_type<T: fmt::Debug>(item: T)
{
    println!("{:?} is {}", item, any::type_name::<T>());
}

//fn compare_and_print<T: fmt::Display+ PartialEq+ From<U>, U: fmt::Display + PartialEq+ Copy>(a: T, b: U)
fn compare_and_print<T, U>(a: T, b: U)

where T: fmt::Display + PartialEq + From<U>,
      U: fmt::Display + PartialEq + Copy

{
    if a == T::from(b)
    {
        println!("{} is equal to {}", a, b);
    }
    else
    {
        println!("{} is not equal to {}", a, b);
    }
}

fn main()
{
    check_type(13);
    check_type(13.0);
    check_type("Sandesh is a programmer!");
    check_type([13,14,38]);
    compare_and_print(10.1, 10);
    compare_and_print(1.0,1);
}

