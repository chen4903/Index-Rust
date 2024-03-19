fn main() {
    println!("Hello, world!");
}

fn dead() -> ! {
    panic!("发散函数")
}

fn forever() -> ! {
    loop {
        // ....
    }
}