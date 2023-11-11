use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:i32 = input2.trim().parse().expect("Not a valid number");

    if ((b * b) - (4 * a * c)) == 0 {
        println!("There is exactly one root")
    }
    else if ((b * b) - (4 * a * c)) > 0 {
        println!("There are two distinct roots.");
    }
    else if ((b * b) - (4 * a * c)) < 0 {
        println!("There are no roots");
    }





}
