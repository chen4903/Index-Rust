# match与if-let

## match

match需要匹配到每一种情况，不能够遗漏，`_`符号就很有用了。

基本用法

```rust
enum Direction {
    East,
    West,
    North,
    South
}

fn main() {
    //////////////////////////////   1.匹配值true   //////////////////////////////
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => println!("South or North"),
        _ => {
            println!("west")
        }
    };

    //////////////////////////////   2.作为返回值true   //////////////////////////////
    return_value(dire);

    //////////////////////////////   3.赋值true   //////////////////////////////
    let world = Direction::South;
    let _hello = match world {
        Direction::South => 12,
        _ => 31231
    };
}

fn return_value(direction: Direction) -> u128 {
    match direction {
        Direction::East => {
            println!("test");
            123
        }
        _ => 12345
        // 或者使用任何一个变量名取代`_`，比如：other => 12345
    }
}
```

嵌套enum

```solidity
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let num = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    };
}
```

复杂一点

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("LEVI_104".to_string()),
        Action::MoveTo(1, 2),
    ];
    for action in actions {
        match action {
            Action::MoveTo(x, y) => {
                // do sth
            },
            Action::Say(s) => println!("hello"),
            _ => {}
        }
    }
}
```

## if-let

有时会遇到只有一个模式的值需要被处理，其它值直接忽略的场景，如果用 `match` 来处理就要写成下面这样：

```rust
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
```

我们只想要对 `Some(3)` 模式进行匹配, 不想处理任何其他 `Some<u8>` 值或 `None` 值。但是为了满足 `match` 表达式（穷尽性）的要求，写代码时必须在处理完这唯一的成员后加上 `_ => ()`，这样会增加不少无用的代码。

俗话说“杀鸡焉用牛刀”，我们完全可以用 `if let` 的方式来实现：

```rust
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }
```

## matches!

`matches!`宏，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 `true` or `false`。

```rust
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let eg_matches = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    let _ = eg_matches.iter().filter(|x| matches!(x, MyEnum::Foo));
}
```







