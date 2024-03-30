# Trait

## 特点

- 孤儿规则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的。比如：无法在当前作用域中，为 `String` 类型实现 `Display` 特征，因为它们俩都定义在标准库中

## 使用

1.你可以类比接口

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

2.接口可以有默认实现，并且有默认实现的方法，在实现trait的时候可以不实现(override)

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Post {}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

3.使用trait作为函数参数：只要是实现了这个trait的参数，都可以传入

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

其实这是个语法糖，完整写法应该这样：

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

4.多重trait：

```rust
pub fn notify1(item: &(impl Summary + Display)) {} // 语法糖形式
pub fn notify2<T: Summary + Display>(item: &T) {}
```

5.where trait：也就是要满足的trait很多，可以用这种形式比较容易

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

6.只有实现了特定trait的参数才能调用方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> { // 所有的Pair结构体都可以使用
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // 只要实现了Display + PartialOrd的结构体才可以使用
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

7.返回值是impl trait：因为 `Weibo` 实现了 `Summary`，因此这里可以用它来作为返回值。要注意的是，虽然我们知道这里是一个 `Weibo` 类型，但是对于 `returns_summarizable` 的调用者而言，他只知道返回了一个实现了 `Summary` 特征的对象，但是并不知道返回了一个 `Weibo` 类型。

```rust
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```

这种 `impl Trait` 形式的返回值，在一种场景下非常非常有用，那就是返回的真实类型非常复杂，你不知道该怎么声明时(毕竟 Rust 要求你必须标出所有的类型)，此时就可以用 `impl Trait` 的方式简单返回。例如，闭包和迭代器就是很复杂，只有编译器才知道那玩意的真实类型，如果让你写出来它们的具体类型，估计内心有一万只草泥马奔腾，好在你可以用 `impl Iterator` 来告诉调用者，返回了一个迭代器，因为所有迭代器都会实现 `Iterator` 特征。

但是这种返回值方式有一个很大的限制：只能有一个具体的类型。例如：编译报错，因为它返回了两个不同的类型 `Post` 和 `Weibo`。

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Weibo {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        }
    }
}
```

## derive

被 `derive` 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

例如 `Debug` 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 `println!("{:?}", s)` 的形式打印该结构体的对象。

再如 `Copy` 特征，它也有一套自动实现的默认代码，当标记到一个类型上时，可以让这个类型自动实现 `Copy` 特征，进而可以调用 `copy` 方法，进行自我复制。

总之，`derive` 派生出来的是 Rust 默认给我们提供的特征，在开发过程中极大的简化了自己手动实现相应特征的需求，当然，如果你有特殊的需求，还可以自己手动重载该实现。

## 实际例子

1.为了实现两个结构体可以用+来相加，我们手动实现Add这个trait

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { // x和y都必须实现Add这个trait，两个东西相加都要实现，不然咋相加，你说是吧？
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T>{
        Point { 
            x: self.x + p.x, 
            y: self.y + p.y
        }
    }
}

fn main() {
    let p1 = Point{x: 1.1, y: 1.2};
    let p2 = Point{x: 3.4, y: 3.1};
    let p3 = p1 + p2;

    println!("p1 add p2 = {:?}", p3);
}
```

2.想debug的时候可以有自定义输出了(而不是使用`#[derive(Debug)]`的默认实现)，需要手动实现`std::fmt::Display`这`trait`

```rust
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState{
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "自定义实现Display: <{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {

    let f6 = File::new("hello.txt");
    println!("{:?}", f6); // 输出了#[derive(Debug)]的默认实现：File { name: "hello.txt", data: [], state: Closed }
    println!("{}", f6); // 输出了自定义实现：<hello.txt (CLOSED)>
}
```







