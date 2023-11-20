fn main() {
    use std::io;

// Define a struct to represent a menu item
struct MenuItem {
    name: String,
    price: f64,
}

// Function to calculate the total price of selected items
fn calculate_total(selections: &Vec<&MenuItem>) -> f64 {
    selections.iter().map(|item| item.price).sum()
}

fn main() {
    // Define menu items
    let menu: Vec<MenuItem> = vec![
        MenuItem { name: String::from("Burger"), price: 5.99 },
        MenuItem { name: String::from("Pizza"), price: 8.99 },
        MenuItem { name: String::from("Salad"), price: 4.99 },
        // Add more items as needed
    ];

    // Display the menu
    println!("Menu:");
    for (index, item) in menu.iter().enumerate() {
        println!("{}. {} - ${}", index + 1, item.name, item.price);
    }

    // Prompt user for selections
    let mut selections: Vec<&MenuItem> = Vec::new();
    loop {
        println!("Enter the item number to add to your order (0 to finish):");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse input as usize
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        // Check if the user wants to finish ordering
        if choice == 0 {
            break;
        }

        // Check if the choice is a valid menu item
        if choice > 0 && choice <= menu.len() {
            selections.push(&menu[choice - 1]);
            println!("Added {} to your order.", menu[choice - 1].name);
        } else {
            println!("Invalid item number. Please choose a number between 1 and {}.", menu.len());
        }
    }

    // Calculate and display the total price
    let total_price = calculate_total(&selections);
    println!("Your total order price is: ${:.2}", total_price);
}

}
