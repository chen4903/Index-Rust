fn main() {
    ////////////////////   1.切片true   ///////////////////////////////
    // let hello = "中国人";

    // let s = &hello[0..3];
    // println!("{}",s);


    ////////////////////   2.字符串操作true   ///////////////////////////////
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    
    // 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}
