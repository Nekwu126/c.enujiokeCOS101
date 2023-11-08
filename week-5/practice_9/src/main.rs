fn main() {
    let mut count = 0;

    for num in 20..60 {
        if num < 25 {
            println!("{:?}",num);
            continue;

        }
        count+=1;

    }
    println!("The count of values greater than 25 (between 20 and 60) is: {}", count);
    //outputs 10
}
