# 解构option

没啥好说的

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

只需要记住Some和None是Option枚举下面的就够了

```rust
enum Option<T> {
    Some(T),
    None,
}
```































