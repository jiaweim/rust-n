# Rust 函数

- [Rust 函数](#rust-函数)
  - [语句和表达式](#语句和表达式)
    - [语句](#语句)
    - [表达式](#表达式)
    - [示例](#示例)
  - [函数简介](#函数简介)
  - [参数](#参数)
  - [返回值](#返回值)
    - [无返回值](#无返回值)
    - [发散函数](#发散函数)

Last updated: 2023-10-17, 14:39
add: 发散函数
2023-10-10, 10:07
@author Jiawei Mao
****
## 语句和表达式

Rust 函数由一系列语句组成，最后由一个表达式返回值。例如：

```rust
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}
```

语句执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。

函数体由一系列语句（statement）组成，结尾可以包含一个表达式（expression）：

- 语句（Statement）：执行某些操作但不返回值的指令
- 表达式（Expression）：求值返回结果

### 语句

创建变量并用 `let` 关键字为其赋值就是语句。示例：

```rust
let a = 8;
let b: Vec<f64> = Vec::new();
let (a, c) = ("hi", false);
```

函数定义也是 statement，上面整个例子就是一个 statement。

语句不返回值，因此不能将语句赋值给另一个变量。例如，下面是**错的**：

```rust
fn main() {
    let x = (let y = 6);
}
```

`let y = 6` 语句不返回值，因此没有任何东西可以绑定 `x`。不像 C 语言中，可以写 `x = y = 6`，让 x 和 y 的值都是 6。

### 表达式

表达式返回一个值。例如 `5+6` 是一个表达式，返回值为 `11`。

- 表达式可以是语句的一部分，如 `let y = 6;`
- 调用函数是表达式，因为会返回值
- 调用宏是表达式
- 用大括号创建的新的 scape 也是表达式，例如

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

该 block 的值是 4，该值赋值给 `y`。注意，`x+1` 行末没有分号，如果添加分号，就转换为了语句，就不返回值。

表达式如果不返回值，会隐式返回一个 `()`：

```rust
fn main() {
    // 调用函数是表达式，但 `ret_unit_type` 不返回值，因此隐式返回 ()
    assert_eq!(ret_unit_type(), ()) 
}
fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}
```

### 示例

- 表达式作为 scope 返回值

```rust
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x // 该语句作为当前 scope 的返回值
    };

    assert_eq!(v, 3);
}
```

- 表达式作为函数返回值

```rust
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

## 函数简介

下面是 `add` 函数：

```rust
fn add(i: i32, j: i32) -> i32 {
   i + j
}
```

其结构为：

- 以关键字 `fn` 开始
- 函数名
- 圆括号包含参数
- 大括号表示方法体


![](images/2023-10-17-14-05-27.png){width="360px"}

要点说明：

- 函数名和变量使用蛇形命名法，即所有函数和变量名都是小写，多个单词以下划线分开。
- 函数的位置可以随便放，Rust 不关心在哪里定义函数
- 每个函数参数都需要标注类型

## 参数

定义函数可以包含形参（parameters）：

- **形参**是函数签名中的特殊变量。
- 当函数有形参时，调用它使可以提供具体的值，具体的值称为**实参**（arguments）。
- 在日常交流中，形参和实参往往可以互换使用。
- Rust 是强类型语言，定义函数参数时需要标注类型

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

## 返回值

Rust 函数就是表达式，能够返回值。在箭头后面声明返回类型。

在 Rust 中，函数的返回值就是最后一条表达式的值。当然也可以使用 `return` 关键字可以提前从函数返回，不过大多数函数隐式返回最后一个表达式的值。例如：

```rust
fn plus_five(x:i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(5);
    println!("The value of x is: {}", x);
}
```

`x + 5` 是最后一条表达式，求职后返回一个值。

**示例：** 同时使用表达式和 return

```rust
fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}

fn main() {
    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);
}
```

### 无返回值

单元类型 `()` 是一个零长度的元组，可以用来表示一个函数没有范湖只：

- 函数没有返回值，则返回 `()`
- 通过 `;` 结尾的表达式返回 `()`

**示例：** 隐式返回 `()`

```rust
use std::fmt::Debug;

fn report<T: Debug>(item: T) {
  println!("{:?}", item);
}
```

**示例：** 显式返回 `()`

```rust
fn clear(text: &mut String) -> () {
  *text = String::from("");
}
```

在实际编程中经常会在错误提示中看到 `()`，假如你的函数需要返回一个 `u32` 值，但是你不幸的以 `表达式;` 的方式作为函数的最后一行代码，就会报错：

```rust
fn add(x:u32,y:u32) -> u32 {
    x + y;
}
```

```
error[E0308]: mismatched types // 类型不匹配
 --> src/main.rs:6:24
  |
6 | fn add(x:u32,y:u32) -> u32 {
  |    ---                 ^^^ expected `u32`, found `()` // 期望返回u32,却返回()
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
7 |     x + y;
  |          - help: consider removing this semicolon
```

只有表达式能返回值，而以 `;` 结尾的是语句。

在 Rust 中必须**严格区分表达式和语句**。

### 发散函数

用 `!` 作为函数返回类型，表示函数永不返回。

这种语法往往用做会导致程序崩溃的函数：

```rust
fn dead_end() -> ! {
  panic!("你已经到了穷途末路，崩溃吧！");
}
```

下面的函数创建了一个无限循环，该循环永不跳出，因此函数也永不返回：

```rust
fn forever() -> ! {
  loop {
    //...
  };
}
```
