fn main()
{
    /*
    let a: u8=20;
    let b: f64= 45.505678345783;

    println!("The value of: \na is {}\nb is {:.2}\n", a,b);

    */

    let mut binary_num= 0b1111_0101u8;
    println!("The binary value is: {}", binary_num);
    println!("The actual value in bits is: {:08b}", binary_num);

    // binary representation is :08b format specifiers
    // :.3 for decimal representation

    // Bitwise NOT operators
    binary_num= !binary_num;
    println!("The bitwise not operators for this value is: {:08b}", binary_num);

   // Bitwise AND operators
   binary_num= binary_num & 0b1111_0111;
   println!("The bitwise and operators for this value is: {:08b}", binary_num);
   println!("The 6th bit is: {}", binary_num & 0b0100_0000);

}