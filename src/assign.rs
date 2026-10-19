fn main()
{
    /*
    let make_it_work= false;

    // conditional assignment
    let x= if make_it_work {1} else {2.4 as i32};

    println!("The value of x is {}", x);

    // counter in rust using loop 

    let mut counter=0;

    let result= loop
    {
        if counter == 10
        {
            break counter * 10;
        }

        counter +=1;

        println!("The counter value is: {}", counter);
    };

    println!("Loop ends here !");
    println!("The final result expected is: {}", result);

   

    let mut count = 0;
    let letters= ['a', 'b', 'c'];

    while count < letters.len()
    {
        println!("Letter is: {}", letters[count]);
        count += 1;
    }

    println!("The total letters in array are: {} ", count);

     */

     let mut counted= 0;

     let message= ['h', 'e', 'l', 'l', 'o'];

     // tuple with enumerate function in call
     // message.iter().enumerate() 

     for (index, &item) in message.iter().enumerate()
     {
        println!("The item for index {} is {}", index, item);

        if(item == 'e')
        {
            break;
        }

     }


     for number in 1..5
     {
        println!("The number is: {}", number);
     }


     /*
     for item in letters
     {
        println!("The letter is: {}", item);
        counted += 1;
     }

     println!("The total items in array are: {}", counted);

     */


}