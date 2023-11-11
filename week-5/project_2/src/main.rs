use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your years of work experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:f32 = input2.trim().parse().expect("Not a valis number");

    if age >= 40.0 && experience >= 3.0 {
        println!("Your incentive is N1,560,000.");
    }
    else if age >= 30.0 && age <= 40.0 && age >= 3.0 {
        println!("Your incentive is N1,480,000.")
    }

    else if age < 28.0 && experience >= 3.0 {
        println!("The incentive is N1,300,000");
    }

    else if experience < 3.0 {
        println!("Your incentive is N100,000");
    }






    
}
