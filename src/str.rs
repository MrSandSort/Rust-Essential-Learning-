fn main()
{
    /*
    // using string literals we can create string like 
    let mut name= "Sandesh";
    
    // can create string of its type using 

    let mut message= String::from("Earth");
    println!("The message is {}", message);

    message.push_str(" is Home");
    println!("The message is {}", message); 
    */


    let outer_planet: String;
    {
        let mut inner_planet= String::from("Mercury");
        outer_planet= inner_planet.clone();
        inner_planet.clear();
        println!("The inner planet is: {}", inner_planet);
    }

    println!("The outer planet is: {}", outer_planet);

}