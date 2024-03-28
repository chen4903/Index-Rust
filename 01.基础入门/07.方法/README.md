# 方法

## 用法

3种self形式：

- `self` 表示 `Rectangle` 的所有权转移到该方法中，这种形式用的较少
- `&self` 表示该方法对 `Rectangle` 的不可变借用
- `&mut self` 表示可变借用

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { // 给枚举实现方法
    fn call(&self) {
        // 在这里定义方法体
    }
}

impl Circle { // 给结构体实现方法
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字。调用的时候要用`::`来调用
    // 这种方法往往用于初始化当前结构体的实例。你可以把它当作构造器
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

getter方法：`rect1.width`是访问字段值，`rect1.width()`是调用方法

```rust
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    println!("{}", rect1.width());
}
```





















