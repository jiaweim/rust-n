# Rust 函数

- [Rust 函数](#rust-函数)
  - [1. 简介](#1-简介)
  - [2. 参数](#2-参数)
  - [3. Statements and Expressions](#3-statements-and-expressions)
  - [4. 返回值](#4-返回值)

Last updated: 2023-10-10, 10:07
@author Jiawei Mao
****

## 1. 简介

Rust 代码使用 snake_case 命令函数和变量，即所有函数和变量名都是小写，多个单词以下划线分开。示例：

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

定义函数：

- 以关键字 `fn` 开始
- 函数名
- 圆括号包含参数
- 大括号表示方法体

通过输入函数名和括号来调用定义的函数。上例在 `main` 中调用 `another_function` 函数。`another_function` 函数定义在 `main` 前面或后面都可以。

## 2. 参数

定义函数可以包含形参（parameters）：

- **形参**是函数签名中的特殊变量。
- 当函数有形参时，调用它使可以提供具体的值，具体的值称为**实参**（arguments）。
- 在日常交流中，形参和实参往往可以互换使用。

例如，为 `another_function` 添加一个参数：

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

运行程序：

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\variables.exe`
The value of x is: 5
```

在函数签名中，必须声明参数类型。多个参数以逗号隔开，例如：

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## 3. Statements and Expressions

函数体由一系列语句（statement）组成，结尾可以包含一个表达式（expression）：

- 语句（Statement）：执行某些操作但不返回值的指令
- 表达式（Expression）：求值返回结果

创建变量并用 let 关键字为其赋值就是 statement。示例：

```rust
fn main() {
    let y = 6;
}
```

函数定义也是 statement，上面整个例子就是一个 statement。

statement 不返回值，因此不能将 let 语句赋值给另一个变量。例如，下面是**错的**：

```rust
fn main() {
    let x = (let y = 6);
}
```

`let y = 6` 语句不返回值，因此没有任何东西可以绑定 `x`。不像 C 语言中，可以写 `x = y = 6`，让 x 和 y 的值都是 6。

Expression 计算为一个值。例如 `5+6` 是一个 expression，计算值为 `11`。

- Expression 可以是 statement 的一部分，如 `let y = 6;`
- 调用函数是 expression
- 调用 macro 是 expression
- 用大括号创建的新的 scape 也是 expression，例如

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

下面的表达式：

```rust
{
    let x = 3;
    x + 1
}
```

该 block 的值是 4，该值赋值给 `y`。注意，`x+1` 行末没有分号，如果添加分号，就转换为了 statement，就不返回值。

## 4. 返回值

函数可以返回值，必须在箭头后面声明返回类型。在 Rust 中，函数的返回值与函数体最终 expression 的值是同义的。通过 return 关键字可以提前从函数返回，不过大多数函数隐式返回最后一个 expression 的值。例如：

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

在 five 函数中没有函数调用、宏，甚至没有 let 语句，只有一个数字 `5`。这里使用 `-> i32` 指定返回类型，运行该示例：

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

其中 `5` 是 `five` 的返回值，对应 `i32` 类型。

再看一个示例：

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

运行该示例，输出 `The value of x is: 6`。

如果在 `x + 1` 后面添加分号，从 expression 转换为 statement，使得 plus_one 不返回值，程序会报错。

