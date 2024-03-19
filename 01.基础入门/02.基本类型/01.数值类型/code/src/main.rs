use num::complex::Complex;

fn main() {
    //////////////////////     1.整数溢出的解决方案true      /////////////////////////
    // let a : u8 = 255;
    // let b = a.wrapping_add(20);
    // // let b = a + 19; 使用这个，在编译期间会报错（attempt to compute `u8::MAX + 19_u8`, which would overflow），但是我的IDE没给出提示
    // println!("{}", b);  // 19

    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
    // assert_eq!(200u8.saturating_add(99), u8::MAX);

    //////////////////////     2.浮点数的问题false      /////////////////////////
    // assert_eq!(0.1+0.2, 0.3); // 不相等

    //////////////////////     3.NaN false      /////////////////////////
    // let x = (-21321312_f32).sqrt();
    // assert_eq!(x,x); // left: NaN, right: NaN

    //////////////////////     4.NaN true      /////////////////////////
    // let x = (-21321312_f32).sqrt();
    // if x.is_nan() {
    //     println!("未定义的数学行为");
    // }

    //////////////////////     5.序列 true      /////////////////////////
    // for i in 1..=5 {
    //     println!("(=):{}",i);
    // }

    // for i in 1..5 {
    //     println!("{}",i);
    // }

    //////////////////////     6.num true      /////////////////////////
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
