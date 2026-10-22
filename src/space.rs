fn main()
{
    /*
    let words= String::from("Greeting from Hello World!");
    let val= &words.as_bytes();

    for (index, &items) in val.iter().enumerate()
    {
        if items == b' '
        {
            println!("value is {:?}", &words[..index]);
            break;
        }
    }
    */

    let sentences= String::from("Greeting from Nepal");
    let rest= except_first(&sentences);
    println!("The rest words after first are: {}",rest );

}

fn except_first(sent: &String)-> &str 
{
    let val = sent.as_bytes();

    for(index, &item) in val.iter().enumerate()
    {
        if item == b' '
        {
            return &sent[index..]
        }
    }

    return &sent;
}