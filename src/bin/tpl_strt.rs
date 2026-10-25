struct Color(u8, u8, u8);


impl Color
{
   fn get_red(&self)-> u8
   {
    return self.0;
   }
   
}


fn main()
{
    let color = Color(255,0,0);
    println!("The red color is: {}", color.get_red());
}