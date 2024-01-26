use std::io::Read;

fn main() {
    //in order to avoid mistakes from the companies when putting in their data,
    //I have decided to put the usernames in a vector, so that they'll just have to
    //input a number to access the username
    loop{
        let u = vec!["cadb", "cham", "dang", "flou", "nest", "unil", "hone", "nige"];


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the number representing your username: (the first four letters of your company)
                                              0 = Cadbury Nigeria Plc
                                              1 = Champion Breweries Plc
                                              2 = Dangote Sugar refinery Plc
                                              3 = Flour Mills Nigeria Plc
                                              4 = Nestle Nigeria Plc
                                              5 = Unilever Nigeria Plc
                                              6 = Honeywell Nigeria Plc
                                              7 = Nigerian Breweries Plc
                                               ");
    std::io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid input");
        let index:usize = input1.trim().parse().expect("Invalid input");
        let ch:&str = u[index];

        print!("{} is the character for your username [{}]\n", ch, index);

    println!(
        "Enter your password: ( The requirements for the password are as follows:
                                         letters between a-z
                                         numbers between 0-9
                                         no uppercase letters
                                         no characters from [$#@] ) "
    );
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid input");
    let mut file = std::fs::File::open("company_stuff.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    

    //initialize the structure
    let comp1 = Company {
        shares: 18000000.00,
        liabilities: 5500000.00,
        lev: 63.33,
        lev_percent: 63.33 * 0.05,    
    };
    let comp2 = Company {
        shares: 25000000.00,
        liabilities: 8000000.00,
        lev: 68.00,
        lev_percent: 68.00 * 0.05,
    };

    let comp3 = Company {
        shares: 10000000.00,
        liabilities: 5500000.00,
        lev: 44.44,
        lev_percent: 44.44 * 0.05,
    };
    let comp4 = Company {
        //username:String::from("flou"),
        //company:String::from("Flour Mills Nigeria Plc"),
        //year:1960,
        shares: 32000000.00,
        liabilities: 4000000.00,
        lev: 87.50,
        lev_percent: 87.50 * 0.05,
    };
    let comp5 = Company {
        shares: 8000000.00,
        liabilities: 1500000.00,
        lev: 81.00,
        lev_percent: 81.00 * 0.05,
    };

    let comp6 = Company {
        shares: 37000000.00,
        liabilities: 11000000.00,
        lev: 70.27,
        lev_percent: 70.27 * 0.05,
    };
    let comp7 = Company {
        shares: 34000000.00,
        liabilities: 9000000.00,
        lev: 73.53,
        lev_percent: 73.53 * 0.05,
    };
    let comp8 = Company {
        shares: 30000000.00,
        liabilities: 12000000.00,
        lev: 60.00,
        lev_percent: 60.00 * 0.05,
    };
    //passing the comps to the function display()
    display(comp1);
    display(comp2);
    display(comp3);
    display(comp4);
    display(comp5);
    display(comp6);
    display(comp7);
    display(comp8);
    
}

use std::io::Write;
fn display(comp: Company) {
    if comp.shares > 20000000.00 {
        let intro = "Company leverages\n";
        let mut file = std::fs::File::create("company.txt").expect("create failed");
        file.write_all(
            "Leverages
                    Cadbury Nigeria Plc = 63
                    Champion Breweries Plc = 68
                    Dangote Sugar Refineries= 44
                    Flour Mills Nigeria Plc = 88
                    Nestle Nigeria Plc= 81
                    Unilever Nigeria Plc = 74
                    Nigerian Breweries Plc = 60"
                .as_bytes(),
        )
        .expect("write failed");
        file.write_all(intro.as_bytes()).expect("write failed");
        println!("Date written to file");
    }

    if comp.liabilities > 10000000.00 {
        println!(
            "The values for the companys' leverages multiplied by 5% are :
                                     Cadbury Nigeria Plc: {}
                                     Champion Breweries Plc: {}
                                     Dangote Sugar Refinery Plc:{}
                                     Flour Mills Nigeria Plc: {}
                                     Nestle Nigeria Plc: {}
                                     Unilever Nigeria Plc: {}
                                     Honeywell Nigeria Plc: {}
                                     Nigerian breweries Plc: {}",
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent,
            comp.lev_percent
        );
    }
}
//declaring a structure
struct Company {
    shares:f64,
    liabilities:f64,
    lev:f32,
    lev_percent:f32,
}
}
//I've put the programme in a loop in order to allow multiple
//people to access the data