fn main()
{
    let mut rocket_fuel= String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("The rocket fuel is: {} and length is {}", rocket_fuel, length);
    slice();
}

// pass length as well
// borrowing + mutable references

fn process_fuel(propellant: &mut String)-> usize
{
    println!("processing propellant: {}", propellant);
    propellant.push_str(" is highly inflammable.");
    let new_fuel= String::from("LNG-2");
    let length= propellant.len();
    return length;
}

fn slice()
{
    let planets= [1,2,3,4,5,6,7,8];

    let inner_planets: &[i32]= &planets[..4];

    println!("The inner planets are: {:?}",inner_planets );
}