fn main() {
    //////////////    1.简单例子true      ////////////////
    // let x = 5;
    // let y = &x;

    // assert_eq!(5,x);
    // assert_eq!(5,*y);

    //////////////    2.不可变引用参数传递给函数true      ////////////////
    // let s1 = String::from("hello, I am LEVI_104");
    // let len = cal_length(&s1);
    // println!("{} length: {}", s1, len);

    //////////////    3.可变引用true      ////////////////
    // let mut s = String::from("hello");
    // let test = String::from("test");
    // change(&mut s);
    // println!("{}", s);
    // println!("{}", s);
    // println!("{}", test);
    // println!("{}", test);


    //////////////    4.同时存在可变&不可变引用false      ////////////////
    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);

    //////////////    5.悬垂引用alse      ////////////////
    let reference_to_nothing = dangle();
}


fn cal_length(s: &String) -> usize {
    s.len()
}

fn change(stuff: &mut String) {
    stuff.push_str(",LEVI_104")
}

fn dangle() -> &String {
    let s = String::from("hello, LEVI_104");
    &s
}