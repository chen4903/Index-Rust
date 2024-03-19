fn main() {
    ////////////////////   1.表达式true        ////////////////////
    // let y = {
    //     let x = 3;
    //     x+1
    // };
    // println!("y: {}", y)

    ////////////////////   2.if也是表达式true        ////////////////////
    assert_eq!(ret_uint_type(), ()); // 函数的返回值是()
}

fn ret_uint_type() {
    let x = 1;
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    let z = if x % 2 == 1 {"odd"} else {"even"};
}