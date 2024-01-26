//declare a structure
struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    //initialize a structure
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::new(),
        age: 56,
    };
    let emp2 = Employee {
        company: String::from("Google Inc."),
        ceo: String::new(),
        age: 51,
    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}
//fetch values of specific structure fields using the
//operator and print it to the console
fn display(emp: Employee) {
    println!("Who is your ceo: ");
    std::io::stdin().read_line(&mut emp.ceo).expect("Failed to read input");
    if emp.ceo = "Satya" {
        println!("name is {}, age is {}, company is {}", emp.ceo, emp.age, emp.company);

    }
    else if emp.ceo = "Bob" {
        println!("name is {}, age is {}, company is {}", emp.ceo, emp.age, emp.company);
    }
    
    
}
