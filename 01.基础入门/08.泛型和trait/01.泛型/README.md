# 泛型

## 枚举

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 结构体&方法

```rust
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

fn main() {
    let hello = Point{x: 1,y: 2};
    let world = Point{x: 1,y: 2};
    println!("p.x = {}", hello.hello(world));
}
```

## const泛型

> Rust 1.51之后引入

下面是可以打印任意长度任意类型数组的功能：

```rust
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [u32; 2] = [1, 2];
    display_array(&arr);
}
```

上面是通过引用实现的，但如果不通过引用呢？用const泛型：

```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) { // N用来替代数组的长度
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
```

const 泛型表达式：假设我们某段代码需要在内存很小的平台上工作，因此需要限制函数参数占用的内存大小，此时就可以使用 const 泛型表达式来实现

```rust
// 目前只能在nightly版本下使用
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn something<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
{
    //
}

fn main() {
    something([0u8; 0]); // ok
    something([0u8; 512]); // ok
    something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}

// ---

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}
```

## 性能

- 在 Rust 中泛型是零成本的抽象
- 在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小

举个例子：

```rust
let integer = Some(5);
let float = Some(5.0);
```

展开后：为Option实现单态化

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```











































