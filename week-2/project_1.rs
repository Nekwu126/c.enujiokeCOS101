fn main() {
	let p:f64 = 52_0000000.00;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let cl = a - p;
	println!("Simple Interest is {}", cl);

}