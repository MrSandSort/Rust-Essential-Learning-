
#[derive(Debug)]

enum Shape
{
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape
{
    fn get_perimeter(&self)-> f64
    {
        match *self
        {
            Shape::Circle(r) => 2.0 * r* std::f64::consts::PI,
            Shape::Rectangle(l,b) => 2.0* l + 2.0 * b,
            Shape::Triangle(a, b, c) => a+b+c 
        }
    }
}

fn main()
{
    let shape= Shape::Rectangle(20.05, 30.50);
    println!("The shape is: {:?}", shape);

    let perimeter= shape.get_perimeter();
    println!("The perimeter of shape is: {:?}", perimeter);
}