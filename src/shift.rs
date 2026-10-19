fn main()
{
    /*
    let mut val: u8= 0b0010_0000;// 64 

    println!("The decimal value is: {}", val);
    println!("The value in binary is: {:08b}", val);
   
    val=  val<<3;

    println!("Shift the bits to the rights: {:08b}",val);
    println!("The decimal value after shift is: {}", val);

    */

    // Boolean operations



    /*
    let a= true;
    let b= false;


    println!("The value of a is {} while b is {}", a,b);
    println!("NOT a is: {}", !a);
    println!("The a AND b is: {}", a & b );
    println!("The a OR b is: {}", a | b);
    println!("The a XOR b is {}", a ^ b);

    let c= (a ^ b) | (a & b);
    println!("The result of c is: {}", c);

    */

    let letter= 'a';
    let number= '1';

    let symbol= '\u{261D}';

    println!("The letter is: {}", letter as u8);
    println!("The number is: {}", number as u8);
    println!("The symbol for finger pointing: {}", symbol);

}