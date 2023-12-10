use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("Lager_drinks.txt").unwrap();
    let mut lager = String::new();
    file.read_to_string(&mut lager).unwrap();
    println!("{}", lager);

    let mut file = std::fs::File::open("Stout_drinks.txt").unwrap();
    let mut stout = String::new();
    file.read_to_string(&mut stout).unwrap();
    println!("\n{}", stout);

    
    let mut file = std::fs::File::open("Non_alcoholic_drinks.txt").unwrap();
    let mut non_alcohol = String::new();
    file.read_to_string(&mut non_alcohol).unwrap();
    println!("\n{}", non_alcohol);








}
