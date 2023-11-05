fn main() {

    let full_name = "Chibudum John Umeh";
    let department = "Computer science";
    let uni = "Pan-Atlantic University";

    
    let mut school = "School of Science".to_string();
    //push string 
    school.push_str("and Technology");
    
    println!("My name is {}",full_name);
    // check length 
    println!("The length of my full_name is: {}",full_name.len());
    println!("I am a student of {} Department", department);
    println!("{}",school);
    println!("{}",uni)



    
}
