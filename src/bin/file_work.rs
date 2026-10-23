use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main()
{
    /*
    // read_to_string methods reads the content of the file to string
    let file_content= fs::read_to_string("message.txt").unwrap();
    println!("The file contents are: {}", file_content);

    // while iterating over each line we can use .lines() method as well
    for line in file_content.lines()
    {
        println!("Line is: {}", line);
    }

    let content= fs::read("message.txt").unwrap();
    println!("File contents: {:?}", content);
    */

    let mut data= String::new();
    data.push_str("Hey, it is freezing cold outside\n");
    data.push_str("Why not work about ongoing challenges\n");
    data.push_str("He has been acknowledged to that bases\n");
    data.push_str("So why worried about yourself?\n");

    fs::write("positive.txt", data);

    let mut file= OpenOptions::new().append(true).open("positive.txt").unwrap();
    writeln!(file, "Why so hurried about life?\n");

}