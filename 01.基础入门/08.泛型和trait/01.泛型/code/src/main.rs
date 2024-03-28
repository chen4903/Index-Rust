struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn hello<U>(&self, other: Point<U>) -> &T { // 结构体定义了一个泛型T，这里面有定义了一个泛型U
        let _ = Point{
            x: 12,
            y: 13
        };
        &self.x
    }
}

impl Point<f32> { // 还能针对特定的具体类型进行定义，如果Point这个泛型类型不是f32，就没有levi()可用
    fn levi(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b // `+`需要两个值实现 std::ops::Add<Output = T> 这个trait
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn display<T: std::fmt::Debug, const N: usize>(arr: [T;N]) {
    println!("{:?}", arr);
}

fn main() {
    let hello = Point{x: 1,y: 2};
    let world = Point{x: 1,y: 2};
    println!("p.x = {}", hello.hello(world));

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [u32; 2] = [1, 2];
    display_array(&arr);

    let arr_levi: [u32; 2] = [1, 2];
    display(arr_levi);
}