#![allow(unused_imports, dead_code)]

// impl & dyn & trait Object
// https://play.rust-lang.org/

trait Nice {
    fn nice(&self);
}

// impl: 编译期间为每一个实现该trait的类，实现一个函数
fn say_nice(n: impl Nice) {
    n.nice();
}

// &dyn：传入一个实现了Nice trait的对象的引用
// 不能去掉'&'，因为dyn是runtime时候确定的，不是编译时候可以确定的，
// 它可大可小，因此数据需要放到堆中，然后在内存使用一个指针指向该对象
fn say_nice_dyn(n: &dyn Nice) {
    n.nice();
}

struct Hello {
    v: i32,
}

struct Good {
    v: i32,
}

impl Nice for Hello {
    fn nice(&self) {}
}

impl Nice for Good {
    fn nice(&self) {}
}

fn main() {
    {
        let h = Hello { v: 30 };
        say_nice(h);

        let g = Good { v: 20 };
        say_nice(g);
    }
    {
        let h = Hello { v: 30 };
        say_nice_dyn(&h);

        let g = Good { v: 20 };
        say_nice_dyn(&g);
    }
}
