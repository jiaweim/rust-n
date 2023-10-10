# 变量和可变性

- [变量和可变性](#变量和可变性)
  - [简介](#简介)
  - [Constants](#constants)
  - [变量掩蔽](#变量掩蔽)

Last updated: 2023-10-09, 20:19
@author Jiawei Mao
****

## 简介

Rust 中变量默认不可变（immutable），这是 Rust 实现高效并发的基础。

不能更改 immutable 变量的值。例如：

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // 报错
    println!("The value of x is: {x}");
}
```

变量前添加 `mut` 使其可变。例如：

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Constants

和 immutable 变量一样，constant 也不能更改，但两者之间有一些区别。

- 不能对常量使用 `mut`
- 常量使用 `const` 关键字声明，而不是 `let`，且必须注释类型
- 最后，常量只能设置为常量表达式，不能设置为在运行时计算的值的结果

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## 变量掩蔽

在 Rust 中可以声明一个与之前变量同名的新变量。称第一个变量被第二个变量掩蔽（shadow）。例如：

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

```
The value of x in the inner scope is: 12
The value of x is: 6
```

shadow 与将标记为 mut 不同，因为如果不使用 let 对变量重新赋值，会得到编译错误。使用 let，不仅可以变换值，还能修改类型。例如：

```rust
let spaces = "   ";
let spaces = spaces.len();
```
