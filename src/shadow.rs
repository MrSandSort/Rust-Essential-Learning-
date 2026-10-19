fn main()
{
    let planet= "Earth"; 
    // in rust we can create same variables again using same name
    {
         println!("The planet is {}", planet);
         let planet= "Mars";
         println!("Now the planet is {}",  planet);
    }

    println!("But outside the scope the planet name is still {}", planet);
   
}