use std::mem;

#[derive (Debug)]

struct Shuttle
{
    name: String,
    crew_size: u8,
    propellant: f64
} 

/*
impl Shuttle
{
    fn get_name(&self)-> &str
    {
        return &self.name;
    }

    fn add_fuel(&mut self, gallons: f64)
    {
        self.propellant += gallons;
    }

    fn new(name: &str)-> Shuttle
    {
        Shuttle
        {
            name: String::from(name),
            crew_size:7,
            propellant: 650.45
        }
    }
}
*/


fn main()
{
    /*
    let mut vehicle = Shuttle::new("Sankii");

    let vehicle_name= vehicle.get_name();
    println!("The name of the vehicle is: {}", vehicle_name);

    println!("The fuel is: {:.2}", vehicle.propellant);
    vehicle.add_fuel(3589.75);

    println!("The current fuel after addition is: {:.2}", vehicle.propellant);

    let vehicle2= Shuttle
    {
        ..vehicle
    };

    vehicle.name= String::from("Lamborghini");

    // println!("The name of this vehicle is: {}", vehicle.name);
    println!("The vehicle is {:?}", vehicle);
    println!("The second vehicle is {:?}", vehicle2);
    */

    let vehicle = Shuttle
    {
        name: String::from("Lamborghini"),
        crew_size: 7,
        propellant: 8790.78
    };

    println!("Size of the vehicle on stack is: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("Size of the boxed vehicle on stack is: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("Size of the boxed vehicle on heap is: {} bytes", mem::size_of_val(&*boxed_vehicle));





}