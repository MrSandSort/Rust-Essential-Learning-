
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]

struct Student {
    id: u32,
    age: u8,
    name: String,
}

fn main() 
{
    let student1 = Student {
        id: 101,
        age: 22,
        name: String::from("Sandesh"),
    };

    let student2 = Student {
        id: 102,
        age: 23,
        name: String::from("Ram"),
    };

    let student3 = Student {
        id: 101,
        age: 22,
        name: String::from("Sandesh"),
    };

    // 1. Equality
    println!("student1 == student3: {}", student1 == student3);

    // 2. Inequality
    println!("student1 != student2: {}", student1 != student2);

    // 3. Greater than
    println!("student2 > student1: {}", student2 > student1);

    // 4. Less than
    println!("student1 < student2: {}", student1 < student2);

    // 5. Greater than or equal to
    println!("student1 >= student3: {}", student1 >= student3);

    // 6. Less than or equal to
    println!("student1 <= student3: {}", student1 <= student3);

    // 7. Ordering using cmp()
    println!("student1 compared with student2: {:?}", student1.cmp(&student2));
}
