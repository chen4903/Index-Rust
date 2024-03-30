fn largetst<T: PartialOrd + Copy>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { // x和y都必须实现Add这个trait，两个东西相加都要实现，不然咋相加，你说是吧？
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T>{
        Point { 
            x: self.x + p.x, 
            y: self.y + p.y
        }
    }
}

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState{
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "自定义实现Display: <{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    // //////////////  1.trait例子true    ///////////////////////////
    // let number_list = vec![25,999,51,32,1,321];
    // let result = largetst(&number_list);
    // println!("The biggest number is {}", result);

    // //////////////  2.两个struct相加true    ///////////////////////////
    // let p1 = Point{x: 1.1, y: 1.2};
    // let p2 = Point{x: 3.4, y: 3.1};
    // let p3 = p1 + p2;

    // println!("p1 add p2 = {:?}", p3);

    //////////////  3.自定义Debug格式true    ///////////////////////////
    let f6 = File::new("hello.txt");
    println!("{:?}", f6); // 输出了#[derive(Debug)]的默认实现：File { name: "hello.txt", data: [], state: Closed }
    println!("{}", f6); // 输出了自定义实现：<hello.txt (CLOSED)>
}
