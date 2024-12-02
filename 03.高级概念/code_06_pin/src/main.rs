use std::{marker::PhantomPinned, mem, pin::pin};

struct SelfRef {
    name: String,
    name_ref: *const String,
    _pin: PhantomPinned, // 这个是 !Unpin，使得整个结构体也变成了 !Unpin
}

impl SelfRef {
    #[inline(always)]
    fn new(name: impl Into<String>) -> Self {
        let mut ret = SelfRef {
            name: name.into(),
            name_ref: std::ptr::null(),
            _pin: PhantomPinned,
        };

        // &raw const表示引用
        ret.name_ref = &raw const ret.name;

        return ret;
    }
}

fn main() {
    // 如果 SelfRef 是 Unpin，则就算用 pin! 也不会有作用。因此，我们在Pin之前，要把它弄成 !Unpin
    // Pin 住这个指针指向的“内存”，而不是指针本身
    
    let mut s1 = pin!(SelfRef::new("hello"));
    let mut s2 = pin!(SelfRef::new("world!"));

    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {
        &*s1.name_ref
    });
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {
        &*s2.name_ref
    });

    // Pin 住了之后，拿不到结构体 SelfRef 中的可变引用，也就是修改不了
    mem::swap(&mut s1.name, &mut s2.name);

    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {
        &*s1.name_ref
    });
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {
        &*s2.name_ref
    });

/*
    // 未 Pin 之前输出
    s1: name: hello, name_ref: hello
    s2: name: world!, name_ref: world!
    s1: name: world!, name_ref: hello
    s2: name: hello, name_ref: world!
    // 发现：swap之后，结构体中 name 和 name_ref 对应不上
*/

}
