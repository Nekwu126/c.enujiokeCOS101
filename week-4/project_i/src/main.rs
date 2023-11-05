// Rust program to calculate speed

use std::io;

 
fn main() {
    println!("\nCalculating speed.");
    
     //Distance in kilometres
   
    let mut distance_miles = String::new();
    println!("\nEnter the distance in miles.");
        io::stdin().read_line(&mut distance_miles). expect("Failed to read distance");
    let distance_miles:f64 = distance_miles.trim().parse().expect("Input not an integer");
    let distance_km = distance_miles as f64 * 1.609344;
    println!("The distance in km is {}", distance_km);


//Time in hours
println!("\nEnter the time in hours");
let mut time = String::new();
io::stdin().read_line(&mut time).expect("Failed to read time");
let time:f64 = time.trim().parse().expect("Input not an integer");
println!("The time in hours is {}", time);


//To calculate speed (speed = distance/time)
let speed:f64 = distance_km / time;

println!("Speed: {} kilometres per hour", speed);


    


    
}
