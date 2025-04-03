use std::mem;

struct SelfRef {
    name: String,
    name_ref: *const String,
}

impl SelfRef {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            name_ref: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.name_ref = &raw const self.name;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s1 = SelfRef::new("hello");
        s1.init();

        let mut s2 = SelfRef::new("world");
        s2.init();

        println!("s1: name: {}, name_ref: {}", s1.name, unsafe {
            &*s1.name_ref
        });
        println!("s2: name: {}, name_ref: {}", s2.name, unsafe {
            &*s2.name_ref
        });

        println!("===============");

        mem::swap(&mut s1, &mut s2);

        println!("s1: name: {}, name_ref: {}", s1.name, unsafe {
            &*s1.name_ref
        });
        println!("s2: name: {}, name_ref: {}", s2.name, unsafe {
            &*s2.name_ref
        });
    }
}
