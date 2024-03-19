fn main() {
    ////////////////////////    1.mut使用false    ///////////////////////////// 

    // let x = 5;
    // println!("The value of x is: {}", x);
    // 报错，因为x是不可变的
    // x = 6;  cannot mutate immutable variable `x`
    // println!("The value of x is: {}", x);


    ////////////////////////    2.mut使用true    ///////////////////////////// 

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);


    ////////////////////////    3.解构true    ///////////////////////////// 
    // let (a, mut b): (bool,bool) = (true, false);
    // // a = true,不可变; b = false，可变
    // println!("a = {:?}, b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    ////////////////////////    4.解构true    ///////////////////////////// 
    // struct Struct { // 声明了一个结构体
    //     e: i32
    // }

    // let (a,b,c,d,e); // 这个声明需要显式声明变量类型，不写会报错，但是后面用到了这些变量，编译器自动推导出来了
    // (a,b) = (1,2); // 直接将1，2赋值给a，b了
    // [c,..,d,_] = [1,2,3,4,5];
    // Struct {e} = Struct {e:5}; // 将e=5赋值给了e

    // assert_eq!([1,2,1,4,5],[a,b,c,d,e]);

    ////////////////////////    5.常量true    ///////////////////////////// 
    // const I_AM_A_CONSTANT: u32 = 123; // 无法确定是啥类型，因此需要显式声明。编译期间就确定了值

    ////////////////////////    6.变量遮蔽true    ///////////////////////////// 
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner x: {}", x);
    }
    
    println!("outside x: {}", x);

}