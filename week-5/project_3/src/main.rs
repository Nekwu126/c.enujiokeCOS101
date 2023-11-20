use std::io;


fn main() {

   let p = "Poundo Yam/Edinkaiko Soup";
   let f = "Fried rice & chicken";
   let a = "Amala and Ewedu Soup";
   let e = "Eba and Egusi Soup";
   let w = "White Rice and stew";

   let p:f32 = 3200.0;
   let f:f32 = 3000.0;
   let a:f32 = 2500.0;
   let e:f32 = 2000.0;
   let w:f32 = 2500.0;

   loop{


println!(" Menu                            Price
           P = Poundo Yam/Edinkaiko Soup   -N3,200
           F = Fried Rice & Chicken        -N3,000
           A = Amala and Ewedu Soup        -N2,500
           E = Eba and Egusi Soup          -N2,000
           W = White Rice and Stew         -N2,500");


let mut food = String::new();
println!("What would you like to eat: ");
io::stdin().read_line(&mut food).expect("Failed to read input");
let food = food.trim().to_lowercase();

if food == "p" || food == "poundo yam/edinkaiko soup"
{println!("How many portions of {} would you like to buy: ", p);}

else if food == "f" || food == "fried rice & chicken"
{println!("How many portions of {} would you like to buy: ", f);}

else if food == "a" || food == "amala and ewedu soup"{
    println!("How many portions of {} would you like to buy: ", a);
}
else if food == "e" || food == "eba and egusi soup";
{"How many portions of {} would you like to eat:  ", e}

else if food == "w" || food == "white rice and stew";
{ println!("How many portions of {} would you like to eat: ", w)}

else {
    println!("Not on the menu"); continue; 
}
let mut quantity == String::new();
io::stdin().read_line(&mut quantity).expect("Not a valid number.");
let quantity:f64 = quantity.trim().parse().expect("Not a valid input");

if food == p || food == "poundo yam/edinkaiko soup"{
    let calc:f64 = 3200 * quantity;
    if calc > 10000{
        let calc:f64 = calc * 0.95;
        println!("You qualify for the discount, so your new price is {}", calc);
    }
    else{
        println!("Your price is {}", calc)};
}
if food == a || food == "Amala and Ewedu Soup"{
    let calc1:f64 = 3000.0 * quantity;
    if calc1 > 10000{
        let calc1:f64 = calc *0.95;
        println!("You qualify for the discount, so your new price is {}", calc1)
    }
else{
    println!("Your balance is {}", calc1);
}
}
if food ==  || food == "Amala and Ewedu Soup"{
    let calc1:f64 = 3000.0 * quantity;
    if calc1 > 10000{
        let calc1:f64 = calc *0.95;
        println!("You qualify for the discount, so your new price is {}", calc1)
    }
else{
    println!("Your balance is {}", calc1);


    
}




    
}
