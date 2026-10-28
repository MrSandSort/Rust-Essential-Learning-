use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone)]

struct Student 
{
    name: String,
    marks: f64,
    semester: u8,
}

impl fmt::Display for Student
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result
    {
        write!(f, "{} scored {} in his {} semester", self.name, self.marks, self.semester)
    }
}

impl PartialEq for Student  
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.marks == other.marks;
    }
}

impl PartialOrd for Student 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        return self.marks.partial_cmp(&other.marks);
    }
} 

fn comparing_students<T>(a:&T, b:&T)
where T: fmt::Display+ PartialOrd
{
    match a.partial_cmp(b)
    {
        Some(Ordering::Greater)=>
        {
            println!("{} is greater than {}", a, b);
        }
        Some(Ordering::Less)=>
        {
            println!("{} is less than {}", a, b);
        }
        Some(Ordering::Equal)=>
        {
            println!("{} is equal to {}", a, b);
        }

        None => 
        {
             println!("None");
        }

    }
}

fn main()
{
    let student1= Student
    {
        name: String::from("Sandesh"),
        marks: 885.50,
        semester: 6
    };

    println!("Student first details: \n{}", student1);

    let student2= Student
    {
        name: String::from("Sagar"),
        marks: 890.50 ,
        semester: 6
    };

    println!("Student second details: \n{}", student2);
    comparing_students(&student1, &student2);

}