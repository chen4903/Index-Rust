fn main() {
    let s = String::from("hello, I am LEVI_104");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let g = gives_ownership();
    println!("{}", g);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(c: i32) {
    println!("{}", c);
}

fn gives_ownership() -> String{
    String::from("give u a String")
}