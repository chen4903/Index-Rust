# 流程控制

if

```rust
fn main() {
    let condition = 1;
    let number = if condition == 1 { // 所有分支返回的类型要一致
        5
    } else if condition == 2 {
        6
    }else {
        7
    };

    println!("The value of number is: {}", number);
}
```

循环

```rust
	//////////////////// 2.for true   ////////////////////////////
    for i in 1..=5 {
        println!("{}", i);
    }
    
    //////////////////// 3.迭代器 true   ////////////////////////////
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() { 
        println!("第{}个元素是{}", i + 1, v);
    }

    //////////////////// 4.mut迭代器 true   ////////////////////////////
    let mut stuff = ["hello".to_string(), "world".to_string()];
    for value in &mut stuff {
        *value = "test".to_string();
    }
    println!("{:?}", stuff);

    //////////////////// 4.只是想循环10次 true   ////////////////////////////
    for _ in 0..10 {
      // ...
    }

    //////////////////// 5.while true   ////////////////////////////    
    let mut n = 0;
    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }

    //////////////////// 6.死循环 true   ////////////////////////////
    let mut n = 0;

    loop {
        if n > 5 {
            break
        }
        println!("{}", n);
        n+=1;
    }   
```

























