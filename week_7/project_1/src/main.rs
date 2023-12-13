use std::io;

fn trapezuim() {

    let mut base_1 = String::new();
    let mut base_2 = String::new();
    let mut height = String::new();

    println!("Enter the first base:");
    io::stdin().read_line(&mut base_1).expect("Not a valid input");
    let base_1:f64 = base_1.trim().parse().expect("Not a valid number");
    println!("Enter the value for the second base: ");
    io::stdin().read_line(&mut base_2).expect("Not a valid input");
    let base_2:f64 = base_2.trim().parse().expect("Not a valid number");
    println!("Enter the value for the height:");
    io::stdin().read_line(&mut height).expect("Not a valid number");
    let height:f64 = height.trim().parse().expect("Not a valid input");


    let trapezium_area:f64 = height / 2 * ( base_1 + base_2);
    println!("The area of the trapezium is {}", height);

}
fn rhombus(){
    let mut diagonal_1 = String::new();
    let mut diagonal_1 = String::new();

    println!("Enter the first diagonal: ");
    io::stdin().read_line(&mut diagonal_1).expect("Not a valid number");
    let diagonal_1:f64 = diagonal_1.trim().parse().expect("Not a valid input");
    println!("Enter the second diagonal: ");
    io::stdin().read_line(&mut diagonal_1).expect("Not a valid number");
    let diagonal_2:f64 = diagonal_2.trim().parse().expect("Not a valid input");

    let rhombus_area:f64 = height / 2 * (1 / 2 * diagonal_1 * diagonal_2);
    println!("The area of the rhombus is {}", rhombus_area);

}

fn parallelogram() {
    let base = String::new();
    let altitude = String::new();

    println!("Enter the value for the base: ");
    io::stdin().read_line(&mut base).expect("Not a valid number");
    let base:f64 = base.trim().parse().expect("Not a valid interger");
    println!("Enter the value for the altitude:");
    io::stdin().read_line(&mut altitude).expect("Not a valid number");
    let base:f64 = height.trim().parse().expect("Not a valid input");

    let para_area:f64 = base * altitude;
    println!("The area of the rhombus is {}", para_area);

fn cube() {
    let length = String::new();

    println!("Enter the value for the length of a side: ");
    io::stdin().read_line(&mut length).expect("Not a valid number");
    let length:f64 = lenght.trim().parse().expect("Not a valid input");

    let area_cube:f64 = length * length;
    println!("")
}

}
