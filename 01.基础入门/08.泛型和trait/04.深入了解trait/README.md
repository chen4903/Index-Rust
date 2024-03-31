# 深入了解trait

## 关联类型

```rust
impl Iterator for Counter {
    type Item = u32; // 关联类型

    fn next(&mut self) -> Option<Self::Item> { // Self是Counter类型，Item是u32
        // --snip--
    }
}

fn main() {
    let c = Counter{..} // 变量c是Iterator这个trait的实例
    c.next() // 这里也就是返回Counter类型中的u32
}
```

## 默认泛型类型参数

当使用泛型类型参数时，可以为其指定一个默认的具体类型，例如标准库中的 `std::ops::Add` 这个trait：

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

它有一个泛型参数 `RHS`，但是与我们以往的用法不同，这里它给 `RHS` 一个默认值，也就是当用户不指定 `RHS` 时，默认使用两个同样类型的值进行相加，然后返回一个关联类型 `Output`。

可能上面那段不太好理解，下面我们用代码来举例：

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

上面的例子中并没有为 `Point` 实现 `Add<RHS>` 这个特征，而是实现了 `Add` （没有默认泛型类型参数），这意味着我们使用了 `RHS` 的默认类型，也就是 `Self`。换句话说，我们这里定义的是两个相同的 `Point` 类型相加，因此无需指定 `RHS`。

与上面的例子相反，下面的例子，我们来创建两个不同类型的相加：

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

## 调用同名方法

优先调用self上的方法

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) { // Human的Pilot这个trait的fly()
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) { // Human的Wizard这个trait的fly()
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) { // Human本身的fly()
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法
}
```

优先调用trait上的方法

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("hello")
    }
}

fn main() {
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());
}
```

## trait定义中的trait约束

需求：让某个trait A 能使用另一个trait B 的功能

```rust
use std::fmt::Display;

trait OutlinePrint: Display { // 如果你想要实现 OutlinePrin 这个trait，首先你需要实现 Display
    fn outline_print(&self) {
        // 如果没有Display的限制，那么在OutlinePrint这个trait中就不一定能调用to_string()
        let output = self.to_string(); 
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

## 在外部类型上实现外部trait(newtype)

需求：为了绕过孤儿规则

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

但是上面的实现很麻烦，需要这样访问：`self.0。`

Rust 提供了一个trait叫 `Deref`，实现该trait后，可以自动做一层类似类型转换的操作，可以将 `Wrapper` 变成 `Vec<String>` 来使用。这样就会像直接使用数组那样去使用 `Wrapper`，而无需为每一个操作都添加上 `self.0`。

同时，如果不想 `Wrapper` 暴露底层数组的所有方法，我们还可以为 `Wrapper` 去重载这些方法，实现隐藏的目的。

























