# 变量绑定与解构

## 简单的

- `mut`的使用

## 解构

普通解构

```rust
let (a, mut b): (bool,bool) = (true, false);
```

在 [Rust 1.59](https://course.rs/appendix/rust-versions/1.59.html) 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了。

```rust
struct Struct { // 声明了一个结构体
	e: i32
}

let (a,b,c,d,e); // 这个声明需要显式声明变量类型，不写会报错，但是后面用到了这些变量，编译器自动推导出来了
(a,b) = (1,2); // 直接将1，2赋值给a，b了
[c,..,d,_] = [1,2,3,4,5];
Struct {e} = Struct {e:5}; // 将e=5赋值给了e

assert_eq!([1,2,1,4,5],[a,b,c,d,e]);
```

## 常量&变量

- 常量用`const`，变量用`let`
- 常量永远不变，在编译期间就确定值

## 变量遮蔽

这和 `mut` 变量的使用是不同的，第二个 `let` 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 `mut` 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。

```rust
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner x: {}", x);
    }
    
    println!("outside x: {}", x);
```

使用场景：如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。比如：

```rust
// 字符串类型
let spaces = "   ";
// usize数值类型
let spaces = spaces.len();
```













