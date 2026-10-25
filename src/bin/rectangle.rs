#[derive(Debug)]
struct Rectangle
{
   width: f64,
   height: f64 
}

impl Rectangle
{
    fn new(width: f64, height: f64) -> Rectangle
    {
        Rectangle
        {
            width,
            height
        } 
    }

    fn get_width(self)-> f64
    {
        return self.width;
    }

    fn get_area(&self) -> f64
    {
        return self.width * self.height;
    }

    fn get_scale(&mut self, scalar_value: f64)
    {
        self.width = self.width * scalar_value;
        self.height= self.height * scalar_value;
    }

}

fn main()
{
    let mut rectangle = Rectangle::new(1.2, 3.4);
    assert_eq!(rectangle.get_area(), 4.08);
    rectangle.get_scale(0.5);
    assert_eq!(rectangle.get_area(), 1.02);
    println!("Test passed..");
    println!("{:?}", rectangle);
}
