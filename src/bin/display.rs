use std::fmt;

#[derive(Debug)]
struct Satellite
{
    name: String,
    velocity: f64
}

impl fmt::Display for Satellite
{
    fn fmt(&self, f: &mut fmt::Formatter <'_> ) -> fmt::Result
    {
        write!(f, "The satellite {} is flying at a speed of {}",self.name, self.velocity)
    }
}

fn main()
{
    let hubble = Satellite
    {
        name: String::from("Hubble Telescope"),
        velocity: 35.89
    };

    println!("The satellite info is: {}", hubble);

}

