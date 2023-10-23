fn main() {
	let p:f64 = 52_0000000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// compound interest
	let a = p * (1.0 + (r/100.0));
	let y = f64::powf(a,n);
	let cl = (a * y) - p;
	println!("Compound Interest is {}", cl);

}