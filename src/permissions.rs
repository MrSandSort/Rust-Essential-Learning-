fn main()
{
    let _read: u8= 0b001;
    let _write: u8= 0b010;
    let _execute: u8= 0b100;

   let mut permissions= _write | _execute;

   if permissions & _write !=0
   {
      println!("User has write permissions");
   }

   else 
   {
      println!("User doesn't have write permissions");
   }

   permissions &= !_write;

   if permissions & _write !=0
   {
      println!("User has write permissions");
   }

   else 
   {
      println!("User doesn't have write permissions");
   }



}