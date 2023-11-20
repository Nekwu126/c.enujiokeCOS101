use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    

    
    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string.");

    println!("Enter your department: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Enter your state of origin: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter your email address: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("Enter your cGPA: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let cgpa:f32 = input5.trim().parse().expect("Not a valid number");
     
    println!("Enter your level: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let level:i32 = input6.trim().parse().expect("Not a valid number");

    println!("Enter your voting number: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let num:f32 = input7.trim().parse().expect("Not a valid number");


    //the conditions for voting
    if cgpa > 4.0 && level > 100 && num < 150.0{
        println!("You are eligible to vote");
    }
    else{
        println!("Sorry, you are not eligible to vote");
    }
         }



     


