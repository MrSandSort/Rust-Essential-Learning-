// this is all about traits in rust which is somewhat similar to interface in java
// we have used default traits as well along with that something is returned in string format



#[derive(Debug)]
struct Student 
{
    name: String,
    age: u8,
}

struct Book 
{
    title: String,
    pages: u32,
}

impl Student
{
    fn new(name: String, age: u8) -> Student
    {
        Student
        {
            name,
            age
        }
    }
}

impl Book 
{
    fn new(title: String, pages: u32) -> Book
    {
        Book
        {
            title,
            pages
        }
    }

}

trait Information
{
    fn basic_info(&self) -> String
    {
        format!("Something is already described.")
    }
}

impl Information for Student
{

}

/*
impl Information for Student
{
    fn basic_info(&self) -> String
    {
        format!("Name: {} and Age: {}", self.name, self.age)
    }
}
*/

impl Information for Book
{
    fn basic_info(&self) -> String
    {
        format!("Title: {} and Pages: {}", self.title, self.pages)
    }
}


fn main() {
    let student = Student::new(String::from("Sandesh"), 22);
    let book = Book::new(String::from("Learning Rust"), 250);

    println!("The student info is {}", student.basic_info());
    println!("The book info is {}", book.basic_info());
}