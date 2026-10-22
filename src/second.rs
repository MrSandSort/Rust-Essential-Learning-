fn main()
{
    let greetings= String::from("Hello from Sandesh!");
    let second_word= find_second(&greetings);

    println!("The second word is: {}", second_word);

}

fn find_second(param: &str)-> &str
{
    let value= param.as_bytes();

    for (index, &item) in value.iter().enumerate()
    {
        if item == b' '
        {
            let rest = &param[index..]; 
            let another_byte= rest.as_bytes();

            for (idx, &itm) in another_byte.iter().enumerate()
            {
                if itm == b' '
                {
                    return &rest[..index]
                }
            }

            return rest; 
        }
    }

    return " ";
}
