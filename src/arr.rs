fn main()
{
    let numbers: [i32; 5];
    numbers= [0; 5]; // repeated 0 with five lengths or full data
    let last_index: usize= numbers.len()-1;
    println!("The last number is: {}", numbers[last_index]);

    // u size depends on processor architecture like 32 bits processor --> 4 bytes
    // while, 64 bits processor --> 8 bytes

    /*
       Multi-dimensional array: array with nested one
       inner dimensional must be same
    */

    let numbers= [[1,2,3], [4,5,6]];

    let num= numbers[1][2];

    //let _garage: [[[i32; 100]; 20];5]; // multidimensional array

    println!("The searching number is:{}", num);

    let mut stuffs: (u8, f32, char) = (10, 3.14, 'x');

    stuffs.0 +=3;

    let first_item= stuffs.0;

    println!("The first element is: {}", first_item);

    let (a,b,c) = stuffs;

    println!("The value of b is: {}", b);

}