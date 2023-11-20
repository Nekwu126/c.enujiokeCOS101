use std::io;

fn main() {
    let mut count = 0;
    while count < 500 {
let mut input1 = String::new();
let mut input2 = String::new();

     println!("Enter your name:");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     
     println!("Enter the number of papers you've published: ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let papers:i32 = input2.trim().parse().expect("Not a valid number");
    
    if 3 <= papers && papers >= 5 {
        println!("Your incentive is N500,000.");
    }
    else if 5 < papers && papers < 10 {
        println!("Your incentive is N1,000,000");
    }
    else if papers < 3{
            println!("Your incentive is N100,000");

        }
        count += 1
    }
        
     

}
