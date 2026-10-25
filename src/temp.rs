fn main()
{
    let celsius_temp= 23.0;
    let fahrenheit_temp= celsius_to_fahrenheit(&celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(temperature_celsius: &f64)-> f64
{
    let temperature_fahrenheit= (1.8 * temperature_celsius)+ 32 as f64;
    return temperature_fahrenheit; 
}