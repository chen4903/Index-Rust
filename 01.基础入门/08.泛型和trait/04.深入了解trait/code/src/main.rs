trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("hello")
    }
}

fn main() {
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());
}