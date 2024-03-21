fn main() {
    let tup = (500, 6.4, 1);
	let _ = tup.0;
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}