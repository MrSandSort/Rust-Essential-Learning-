fn main()
{
    let value= String::from(" We are here for you.  ");
    let trimmed_val= trim_space(&value);
    assert_eq!(trimmed_val, "We are here for you.");
    println!("Test passed!");
}

fn trim_space(element: &str)-> &str
{
    let mut start=0;

    for (index, character) in element.chars().enumerate()
    {
        if character != ' '
        {
            start= index;
            println!("{}",start);
            break;
        }
    }

    let mut end=0;

    for (index, character) in element.chars().rev().enumerate()
    {
        if character != ' '
        {
            end= element.len() - index;
            println!("{}",end);
            break;
        }
    }

    return &element[start..end]; 


}