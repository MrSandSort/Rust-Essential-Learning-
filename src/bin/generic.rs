
// defined the generic struct of type T & U

#[derive(Debug)]
struct Rectangle<T, U>
{
    width: T,
    height: U
}

impl <T, U> Rectangle <T, U>
{
    fn get_width(&self) -> &T
    {
        return &self.width;
    }
}


fn main()
{
    // implemented the generic value of both types

    let mut rectangle= Rectangle
    {
        width: 2u8,
        height: 5u16
    };

    println!("Rectangle is {:?}", rectangle);
    println!("Width is {}", rectangle.get_width());
}