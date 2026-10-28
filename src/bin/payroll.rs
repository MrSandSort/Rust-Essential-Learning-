use std::fmt;

#[derive(Debug, Clone)]
struct Employee 
{
    id: u32,
    name: String,
    department: String,
    salary: f64,
}

impl fmt::Display for Employee
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result
    {
        write!(f,"Employee id is: {}\nEmployee name: {}\nDepartment Name: {}\nSalary{:.2}", self.id, self.name, self.department, self.salary)
    }
}

fn main()
{
    let firstEmployee= Employee
    {
        id: 101,
        name:String::from("Sandesh Paudel"),
        department: String::from("IT"),
        salary: 25535.75
    };

    println!("First employee is:\n {}", firstEmployee);
}


