# Rust项目组织结构

### 基本

- Package：包含一个Cargo.toml，描述了如何构建这些Crates。一个Package可以有多个Binary Crate，但是最多只能有一个Library Crate。一个Package至少包含一个Crate
- Crate
  - Binary
  - Library
- Module
- Path

### workspace

根目录下有一个Cargo.toml文件

```toml
[workspace]

members = [ "add_one","adder"]
```

目录结构：

```
~/code/practice/add ❯ tree -L 2                                                                                                                                                                    03:09:24 PM
.
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
├── adder
│   ├── Cargo.toml
│   └── src
└── target
    ├── CACHEDIR.TAG
    └── debug
```

在根目录下用`cargo build`进行编译；用`cargo test -p add_one`进行测试

这是adder的Cargo.toml

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]

add_one = {path = "../add_one"}
```

这是add_one的`Cargo.toml`

```toml
[package]
name = "add_one"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

