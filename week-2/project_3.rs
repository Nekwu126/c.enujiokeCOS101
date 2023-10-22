fn main() {
	let p:f64 = 21_0000;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	//simple interest
	let a = p * (1.0 + (r / 100.0)) * n;
	println!("Amount is {}", a);
}