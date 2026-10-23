use std::env;
use std::io;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


fn main()
{
    /*
    for (index, argument) in env::args().enumerate()
    {
        println!("Text file path at {} is {}", index, argument);
        println!("Name to search at {} is {}", index);
    }
    */

    if env::args().len() < 2
    {
        println!("Length of command line arguments can't be less than 2");
        return;
    }

    let mut person= String::new();

    person.push_str("Sandesh Paudel\n");
    person.push_str("Sagar Paudel\n");
    person.push_str("Sita Paudel\n");
    person.push_str("Nawaraj Parajuli\n");
    person.push_str("Farad Ali Anshar\n");

    fs::write("persons.txt", person);

    let mut file= OpenOptions::new().create(true).append(true).open("persons.txt").unwrap();
    writeln!(file, "Sangey Dada");

    let file_path = env::args().nth(1).unwrap();
    println!("Text file path is: {}", file_path);

    let search_name = env::args().nth(2).unwrap();
    println!("Name to search is: {}", search_name);

    let mut content = fs::read_to_string(file_path).unwrap();

    let mut is_found= false;

    for name in content.lines() 
    {
        if search_name == name
        {
            is_found = true;
            break;
        }
        else
        {
            is_found = false;
        }
    }

    if is_found
    {
        println!("Name is in the list!");
    }
    else
    {
        println!("Name is not in the list");
    }

}