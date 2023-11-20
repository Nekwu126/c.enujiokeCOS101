use std::io;


fn main() {
    let mut input = String::new();
    println!("Enter n: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let n:f32 = input.trim().parse().expect("Not a valid number");

    for input in..=10 {
         for j in 1..=10 {
            let result = i * j;
            println!("{} * {} = {}", i, j, result);
         }

         println!();
    }


    
}
