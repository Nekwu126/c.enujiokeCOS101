//define the prices of each laptop brand
struct Laptop {
    hp:u32,
    ibm:u32,
    toshiba:u32,
    dell:u32,
}

//logic to calculate the total prices of the laptops
//if the customer purchases three of each
impl Laptop {
    fn prices(&self)->u32 {
        self.hp + self.ibm + self.toshiba + self.dell
    }
}

fn main() {
    //instantiate a structure
    let laptop1 = Laptop {
        hp:650000 * 3,
        ibm:755000 * 3,    
        toshiba:550000 * 3, 
        dell:850000 * 3,
    };
    

//print the laptop prices
println!("\nThe price for three hp laptops is {} 
            the price of three IBM laptops is {}, 
            the price of three Toshiba laptops {}, 
            The price for three dell laptops is{},
            so your total price is {} \n", laptop1.hp, laptop1.ibm, laptop1.toshiba, laptop1.dell, laptop1.prices());
    
}    
    
    
    





