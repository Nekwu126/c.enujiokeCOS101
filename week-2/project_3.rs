fn main() {
	let p:f64 = 21_0000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	//Depreciation
	let a = p * (1.0 - (r / 100.0));
	let y = f64::powf(a,n);
	let cl = a * y;
	println!("Amount is {}", cl);
}