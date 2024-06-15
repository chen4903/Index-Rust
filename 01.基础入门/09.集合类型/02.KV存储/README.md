# KV存储HashMap

- 创建

```rust
use std::collections::HashMap;

let mut my_gems = HashMap::new();
my_gems.insert("Hello", 123);
```

- 迭代器

```rust
fn main(){
  use std::collections::HashMap;
  
  let teams_list = vec![
      ("中国队".to_string(), 100),
      ("美国队".to_string(), 10),
      ("日本队".to_string(), 50),
  ];
  
  let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
  
  println!("{:?}", teams_map)
}
```

- 所有权

```rust
fn main() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    println!("name:{}", name); // 报错
    println!("age: {}", age);
}
```

```rust
fn main() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    std::mem::drop(name);
    println!("handsome_boys: {:?}", handsome_boys); // 报错
    println!("age: {}", age);
}
```

- 获取

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name);
```

- 遍历

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

- 更新

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
```

























