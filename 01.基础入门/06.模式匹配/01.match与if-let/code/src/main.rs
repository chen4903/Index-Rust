enum Direction {
    East,
    West,
    North,
    South
}

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

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

enum MyEnum {
    Foo,
    Bar
}

fn main() {
    //////////////////////////////   1.匹配值判断，然后做一些事情true   //////////////////////////////
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
    let hello = match world {
        Direction::South => 12,
        _ => 31231
    };

    //////////////////////////////   4.嵌套enum true   //////////////////////////////
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

    //////////////////////////////   5.综合enum true   //////////////////////////////
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

    //////////////////////////////   6.if-let true   //////////////////////////////
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }
    //////////////////////////////   7.matches!宏 true   //////////////////////////////
    let eg_matches = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    let _ = eg_matches.iter().filter(|x| matches!(x, MyEnum::Foo));
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