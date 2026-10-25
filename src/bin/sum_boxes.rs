use std::ops;

fn main()
{
    let one= Box::new(1);
    let two= Box::new(2);

    assert_eq!(*sum_of_boxes(one, two), 3);

    let pie= Box::new(3.14159);
    let e= Box::new(2.71828);

    assert_eq!(*sum_of_boxes(pie, e), 5.85987);

    println!("Test passed !");
}

fn sum_of_boxes <T: ops::Add<Output = T> > (a: Box<T>, b: Box<T>) -> Box<T>
{
    return Box::new(*a + *b);   
}