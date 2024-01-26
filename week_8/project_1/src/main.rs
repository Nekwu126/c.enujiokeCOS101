fn main() {

  println!("This programme will be used to validate your level as a member of staff");


  
  println!("These are the public servant positions you may fit into: Office Administrator(oa)
                                                                     Academic(a)
                                                                     Lawyer(l)
                                                                     Teacher(t)
            Type the letters beside the positions to show your levels.");
  let mut input2 = String::new();

  let mut input3 = String::new();
  let mut input4 = String::new();
  let mut input5 = String::new();

  


  let mut input1 = String::new();
  println!("Enter your number of years of experience: ");
  std::io::stdin().read_line(&mut input1).expect("Failed to read input");
  let experience:i32 = input1.trim().parse().expect("Invalid input");
  
  //levels are staff levels
  let office_admin = vec!["Office Intern", "Paralegal lawyer", "Placement teacher"];





  
}
