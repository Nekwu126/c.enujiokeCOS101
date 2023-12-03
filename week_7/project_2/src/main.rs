use std::io;

fn main() {
let mut name = String::new();
let mut age = String::new();
let mut marital_status = String::new();
let mut siblings = String::new();
let mut waec_status = String::new();
    
    println!("Enter the first name: ");
    io::stdin().read_line(&mut name).expect("Not a valid number");
    
    println!("Do you have siblings? ");
    let 

    
    println!("{}, Enter your age: ", name);
    io::stdin().read_line(&mut age).expect("Not a valid number");
    let mut age:i32 = age.trim().parse().expect("Not a valid input");
    
    if age > 18 {
        println!("Are you married or single");
        io::stdin().read_line(&mut marital_status).expect("ïnvalid inpiut");
    } 
    else {
        println!("You are not eligible for this program");
        return;
}
  println!("How many siblings do you have: ");
  io::stdin().read_line(&mut siblings).expect("Not a valid number");
  let mut siblings:i64 = siblings.trim().parse().expect("Not a ")
}

