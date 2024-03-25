# 枚举

## 简介

```rust
#[derive(Debug)]
enum PokerCard { // 枚举类型
    Clubs(u8),
    Spades(u8), // 枚举值
    Diamonds(char),
    Hearts(char),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
   let c1 = PokerCard::Spades(5); // 定义枚举
   let c2 = PokerCard::Diamonds('A');

   print_card(c1);
   print_card(c2);

   let _m1 = Message::Move { x: 12, y: 323 }; // 定义枚举
   let _m2 = Message::Write("Hello, I am LEVI_104".to_string());

}

fn print_card(card: PokerCard) { // 枚举作为形参
    // 需要在定义 enum PokerCard 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",card);
}
```

## 处理空值

> 在其它编程语言中，往往都有一个 `null` 关键字，该关键字用于表明一个变量当前的值为空（不是零值，例如整型的零值是 0），也就是不存在值。当你对这些 `null` 进行操作时，例如调用一个方法，就会直接抛出null 异常，导致程序的崩溃，因此我们在编程时需要格外的小心去处理这些 `null` 空值。

Rust使用 `Option` 枚举变量来表述这种结果

```rust
enum Option<T> {
    Some(T),
    None,
}
```

如果使用 `None` 而不是 `Some`，需要告诉 Rust `Option<T>` 是什么类型的，因为编译器只通过 `None` 值无法推断出 `Some` 成员保存的值的类型。

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```











