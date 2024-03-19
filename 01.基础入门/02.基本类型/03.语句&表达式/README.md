# 语句&表达式

## 语句

```rust
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式，它作为一个返回值i32
}
```

由于 `let` 是语句，因此不能将 `let` 语句赋值给其它值，如下形式是错误的：

```rust
let b = (let a = 8);
// 现在不可以用，以后可能支持
```

## 表达式

调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

表达式不带分号

if语句也是表达式：

```rust
let y = if x % 2 == 1 {
    "odd"
} else {
    "even"
};
let z = if x % 2 == 1 { "odd" } else { "even" };
```

