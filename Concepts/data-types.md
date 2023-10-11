# 数据类型

- [数据类型](#数据类型)
  - [1. 简介](#1-简介)
  - [2. 标量类型](#2-标量类型)
    - [2.1. 整数](#21-整数)
    - [2.2. 浮点数](#22-浮点数)
    - [2.3. 数字操作](#23-数字操作)
    - [2.4. Boolean 类型](#24-boolean-类型)
    - [2.5. 字符类型](#25-字符类型)
  - [3. 复合类型](#3-复合类型)
    - [3.1. Tuple](#31-tuple)
    - [3.2. Array](#32-array)
      - [3.2.1. 访问数组元素](#321-访问数组元素)
      - [3.2.2. 无效数组访问](#322-无效数组访问)

Last updated: 2023-10-09, 21:40
@author Jiawei Mao
****

## 1. 简介

Rust 是静态类型语言，在编译时必须知道所有变量的类型。编译器通常可以根据值及其使用方式推断出我们想使用的类型，在可能有多种类型的情况，必须添加类型注释。例如：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

如果不加 `: u32`，会显示如下编译错误：

```rust
error[E0282]: type annotations needed        
  --> src\main.rs:10:9
   |
10 |     let guess = "42".parse().expect("Not a number!");
   |         ^^^^^
   |
help: consider giving `guess` an explicit type
   |
10 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
   |              ++++++++++++

For more information about this error, try `rustc --explain E0282`.  
```

## 2. 标量类型

标量类型表示单个值。Rust 有 4 种主要的标量类型：

- 整数
- 浮点数
- 布尔值
- 字符

### 2.1. 整数

整数分为两种类型：

- 无符号（unsigned） `u`：只能表示非负数
- 有符号（signed） `i`：可以表示负数

|Length|Signed|Unsigned|
|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

- signed 存储**数字范围**：$[-2^{n-1}, 2^{n-1}-1]$, n 为字节数

所以 `i8` 的数字范围为 $[-2^7,2^7-1]=[-128,127]$。

- unsigned 存储数字范围：$[0, 2^n-1]$

所以 `u8` 的数字范围为 $[0, 2^8-1]=[0,255]$。

isize 和 usize 类型取决于计算机的结构，64-bit 机器上为 64-bit；32-bit 机器上为 32-bit。

整数字面量支持如下形式：

|Number literals|Example|
|---|---|
|Decimal|`98_222`|
|Hex|`0xff`|
|Octal|`0o77`|
|Binary|`0b1111_0000`|
|Byte ( u8 only)|`b'A'`|

!!! note
    可以是多个数值类型的数字字面量可以使用后缀指定类型，如 `57u8`。

数字字面量可以用 `_` 作为分隔符，便于阅读，例如 `1_000`，与 `1000` 值相同。

另外：

- 整数类型默认为 `i32`
- isize 或 usize 主要在某些情况为集合建立索引

!!! warning
    **整数溢出**
    例如，类型为 `u8` 的变量可以保存 [0, 255] 之间的值，将其赋值为 256，就发生整数溢出。整数溢出会出现两种情况：

    - 在 debug 模式，Rust 会检查整数溢出，出现溢出时抛出异常，称为 *panic*
    - 在 `--release` 模式，Rust 不检查整数溢出。发生溢出时，大于该类型最大值时会直接取模。在 `u8` 中，256 变为 0，257 变为 1，依此类推。
    
    标准库为基本类型提供了显式处理溢出的方法：
    
    - 所有模式下都 wrap 的 `wrapping_*` 函数，如 `wrapping_add`
    - 出现溢出时返回 `None` 的 `checked_*` 方法
    - 同时返回值和表示是否溢出的 boolean 值的 `overflowing_*` 方法
    - 在最小值或最大值处饱和的 `saturating_*` 方法

### 2.2. 浮点数

Rust 有两种浮点类型：

- `f32`
- `f64`

默认类型为 `f64`，其速度与 `f32` 大致相同，但精度更高。所有浮点类型都有符号。

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### 2.3. 数字操作

Rust 支持所有基本数学运算：加减乘除和余数。整数除法会向 0 截断到最接近的整数。例如：

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

### 2.4. Boolean 类型

Boolean 类型有 2 个值：true 和 false。

Boolean 类型只有 1 个字节大小，使用 `bool` 注释类型。

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### 2.5. 字符类型

`char` 表示字符类型。示例：

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

`char` 字面量使用单引号，而字符串字面量使用双引号。Rust 中 `char` 类型包含 4 个字节，与 Unicode 标量值对应。

Unicode 标量值范围为：[U+0000, U+D7FF] 和 [U+E000, U+10FFFF]。

## 3. 复合类型

复合类型（compound type）将多个值组合为一种类型。Rust 有两种基本复合类型：Tuple 和 array。

### 3.1. Tuple

元组（tuple）声明方式：括号中，以逗号分隔值。

示例：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

从元组获取值（destructuring）：

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

也可以直接用 `.index` 方式访问 tuple 值：

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

没有值的元组有一个特殊名称：*unit*。该值及其类型都写为 `()`。表示表达式不返回任何值。

### 3.2. Array

数组，与元组不同的是：数组的每个元素类型必须相同。声明数组方式：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当希望将数据保存在栈而不是堆上，或者需要保存元素的数量固定，就使用数组。

数组没有 `vector` 灵活，`vector` 可以增大或缩小。当已知元素个数，推荐使用数组：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

声明时指定类型和大小：

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

声明所有值相同的数组：

```rust
let a = [3; 5];
```

表示数组 `a` 大小为 5，所有元素都是 3，等价于 `let a = [3, 3, 3, 3, 3];`。

#### 3.2.1. 访问数组元素

- 使用数组访问

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

#### 3.2.2. 无效数组访问

下面看看索引超出数组范围，会有什么效果：

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

该代码编译没问题，输入 0, 1, 2, 3, 4，程序会输出对应位置的元素值。如果超出该范围，例如 10，会手粗如下错误信息

```
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10',
src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

编译时 Rust 不可能预知你输入的索引值，所以这是一个运行时错误。

