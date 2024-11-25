#![allow(dead_code, unused)]

use std::{sync::Arc, thread};

use self::nice::hello;

mod nice {
    pub fn hello() {
        println!("Hello!");
    }
}

fn main() {
    hello();
    // let mut h = new(100);
    let mut h = Hello::new(100);
    // print(&h);
    // 等价于：Hello::print(&h);
    h.print();
    // add(&mut h, 1000);
    h.add(1000);
    // print(&h);
    h.print();
    // consume(h);
    h.consume();

    let mut h = Hello::new(100);
    h.hello();
    (&h).hello();

    let abc = Arc::new(Hello::new(10));
    let xyz = abc.clone();
    thread::spawn(move || {
        xyz.hello();
    });
    abc.world();
}

trait Nice {
    fn new_nice(&self) -> Self;
    fn new_a(&self);
    fn new_b(&mut self);
    fn new_c(self);
}

struct Hello {
    v: i32,
}

impl Hello {
    fn new(n: i32) -> Self {
        Hello { v: n }
    }

    fn print(&self) {
        println!("{}", self.v);
    }

    fn add(&mut self, n: i32) {
        self.v += n;
    }

    fn consume(self) {
        println!("{}", self.v);
    }

    /*
        关联函数：impl里面的函数
        方法(method)：第一个参数是self，且self需要可以被解析(dereference)为Self关联函数(也就是Hello)

        `self`

        `&self` -> self: &Hello
        `&mut self` -> self: &mut Hello

        智能指针实现了解析的trait
        `self: Box<Self>`
        `self: Re<Self>`
        `self: Arc<Self>`

        P是指针，因此也可以被解析
        `self: Pin<P>`

    */

    fn hello(self: &Hello) {}
    fn world(self: Arc<Self>) {}
}

fn new(n: i32) -> Hello {
    Hello { v: n }
}

fn print(h: &Hello) {
    println!("{}", h.v);
}

fn add(h: &mut Hello, n: i32) {
    h.v += n;
}

fn consume(h: Hello) {
    println!("{}", h.v);
}
