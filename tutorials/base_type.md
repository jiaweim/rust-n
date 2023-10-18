# 基本类型

- [基本类型](#基本类型)
  - [简介](#简介)
  - [类型推导与标注](#类型推导与标注)
  - [数值类型](#数值类型)
    - [整数](#整数)
    - [整型溢出](#整型溢出)
    - [浮点数](#浮点数)
      - [浮点数陷阱](#浮点数陷阱)
      - [NaN](#nan)
    - [数学运算](#数学运算)
    - [位运算](#位运算)
    - [序列](#序列)
    - [as 类型转换](#as-类型转换)
    - [有理数和复数](#有理数和复数)

Last updated: 2023-10-17, 11:28
@author Jiawei Mao
****

## 简介

Rust 数据类型可以分为两类：基本类型和复合类型。

基本类型包括：

- **数值类型**
  - 有符号整数：i8 , i16 , i32 , i64 , isize
  - 无符号整数：u8 , u16 , u32 , u64 , usize
  - 浮点数：f32 , f64
  - 有理数
  - 复数
- 字符串：字符串字面量和字符串切片 `&str`
- 布尔类型： true 和 false
- 字符类型: 表示单个 Unicode 字符，存储为 4 个字节
- 单元类型: 即 `()` ，其唯一的值也是 `()`

## 类型推导与标注

Rust 是一门静态类型语言，编译器必须在编译期知道所有变量的类型，但这不意味着你需要为每个变量指定类型：

- Rust 编译器可以根据变量的值和上下文中的使用方式来自动推导出变量的类型
- 在编译器无法推导出变量类型时，需要手动标注类型

**示例：** 无法推断出类型

```rust
let guess = "42".parse().expect("Not a number!");
```

编译器无法确定 "42" 是整数、浮点数还是字符串，因此会报错：

```sh
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

此时需要显式标准类型：

```rust
let guess: i32 = "42".parse().expect("Not a number!");
```

或者：

```rust
let guess = "42".parse::<i32>().expect("Not a number!");
```

## 数值类型

Rust 的数值类型与其它语言较为相似，但也有差异：

- Rust 数值类型很多
- 类型转换必须显式，例如 Rust 不会偷偷帮你把 16bit 整数转换为 32 bit 整数

### 整数

整数分为两种类型：

- 无符号（unsigned） `u`：只能表示非负数，存储数字范围为 $[0, 2^n-1]$
- 有符号（signed） `i`：可以表示负数，存储数字范围 $[-2^{n-1}, 2^{n-1}-1]$

例如，`i8` 的数字范围为 $[-2^7,2^7-1]=[-128,127]$；`u8` 的数字范围为 $[0, 2^8-1]=[0,255]$。

|长度|有符号类型|无符号类型|
|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

isize 和 usize 类型取决于程序运行的计算机 CPU 类型：64-bit 机器上为 64-bit；32-bit 机器上为 32-bit。

**示例：** 整形字面量

|数字字面量|示例|
|---|---|
|十进制|`98_222`|
|十六进制|`0xff`|
|八进制|`0o77`|
|二进制|`0b1111_0000`|
|字节 ( u8 only)|`b'A'`|


注意：

- 整数类型默认为 `i32`，为推荐使用类型
- `isize` 或 `usize` 主要用作集合索引

### 整型溢出

类型为 `u8` 的变量可以保存 [0, 255] 的值，如果将其赋值为 256，就发生整数溢出。整数溢出会出现两种情况：

- 在 debug 模式，Rust 会检查整数溢出，出现溢出时抛出异常，称为 *panic*
- 在 `--release` 模式，Rust 不检查整数溢出。发生溢出时，大于该类型最大值时会被补码转换成该类型能够支持大的对应数字的最小值，如在 `u8` 中，256 变为 0，257 变为 1，依此类推。程序不会 panic，但结果不是所期望的。
    
标准库为基本类型提供了显式处理溢出的方法：
    
- 所有模式下都 wrap 的 `wrapping_*` 函数，如 `wrapping_add`
- 出现溢出时返回 `None` 的 `checked_*` 方法
- 同时返回值和表示是否溢出的 boolean 值的 `overflowing_*` 方法
- 在最小值或最大值处饱和的 `saturating_*` 方法

**示例：** `wrapping_add`

```rust
fn main() {
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
}
```

### 浮点数

Rust 有两种浮点类型：

- `f32`
- `f64`

默认类型为 `f64`，其速度与 `f32` 大致相同，但精度更高。所有浮点类型都有符号。

**示例：** 浮点数

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

#### 浮点数陷阱

浮点数由于底层格式的特殊性，导致了如果在使用浮点数时不够谨慎，就可能造成危险，有两个原因：

1. **浮点数往往是你想要表达数字的近似**

浮点数类型是基于二进制实现的，但是我们想要计算的数字往往是基于十进制，例如 `0.1` 在二进制上并不存在精确的表达形式，但是在十进制上就存在。这种不匹配性导致一定的歧义性，更多的，虽然浮点数能代表真实的数值，但是由于底层格式问题，它往往受限于定长的浮点数精度，如果你想要表达完全精准的真实数字，只有使用无限精度的浮点数才行。

2. **浮点数在某些特性上是反直觉的**

例如大家都会觉得浮点数可以进行比较，它们确实可以使用 `>`，`>=` 等进行比较，但是在某些场景下，这种直觉上的比较特性反而会害了你。因为 `f32` ，`f64` 上的比较运算实现的是 `std::cmp::PartialEq` trait(类似其他语言的接口)，但是并没有实现 `std::cmp::Eq` trait，但是后者在其它数值类型上都有定义。

**示例：**

Rust 的 `HashMap` 是一个 KV 类型的 Hash Map 实现，它对于 `K` 没有特定类型的限制，但要求 K 的类型必须实现了 `std::cmp::Eq` trait，因此无法使用浮点数作为 `HashMap` 的 `Key`，但是作为对比，Rust 的整数类型、字符串类型、布尔类型都实现了该特征，可以作为 `HashMap` 的 `Key`。

为了避免上面说的两个陷阱，需要遵守以下准则：

- 避免在浮点数上测试相等性
- 当结果在数学上可能存在未定义时，需要格外的小心

**示例：** 测试浮点数相等性

```rust
fn main() {
  // 断言0.1 + 0.2与0.3相等
  assert!(0.1 + 0.2 == 0.3);
}
```

这段代码会 panic，因为二进制精度问题，导致 `0.1 + 0.2` 并不严格等于 0.3，可能在小数点 N 位后存在误差。

如果确实需要比较，可以考虑如下方式：

```rust
(0.1_f64 + 0.2 - 0.3).abs() < 0.00001
```

#### NaN

数学上未定义的结果，例如给负数取平方根 `-42.1.sqrt()`，在 Rust 中返回 `NaN` (not a number)。

所有跟 `NaN` 交互的操作，都返回 `NaN`，而且 `NaN` 不能用来比较，否则 panic。

**示例：** 比较 `NaN` 导致 panic

```rust
fn main() {
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x);
}
```

`assert_eq!(x, x)` 比较 NaN，所以 panic。

可以用 `is_nan()` 检查是否为 NaN：

```rust
fn main() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}
```

### 数学运算

Rust 支持所有基本数学运算：加、减、乘、除和余数。整数除法会向 0 截断到最接近的整数。例如：

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // 求余
    let remainder = 43 % 5;
}
```

**示例：** 各种表示展示

```rust
fn main() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
    // 通过类型后缀的方式进行类型标注：23是i32类型
    let twenty_three = 23_i32;
    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two + twenty_three;
    println!("{} + {} + {} + {} = {}", twenty, twenty_one, twenty_two, twenty_three, addition);

    // 对于较长的数字，可以用 _ 进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}
```

```sh
20 + 21 + 22 + 23 = 86
1000000000000
42.00
```

### 位运算

Rust 的位运算和其他语言基本一样

|运算符|操作|说明|
|---|---|---|
|`&` |位与 |相同位置均为1时则为1，否则为0|
| `\|`|位或 |相同位置只要有1时则为1，否则为0|
|`^` |异或 |相同位置不相同则为1，相同则为0|
|`!` |位非 |把位中的0和1相互取反，即0置为1，1置为0|
|`<<`| 左移| 所有位向左移动指定位数，右位补0|
|`>>`| 右移| 所有位向右移动指定位数，带符号移动（正数补0，负数补1）|

**示例：** 位运算

```rust
fn main() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {} ", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    let mut a = a;

    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}
```

```
(a & b) value is 2
(a | b) value is 3
(a ^ b) value is 1
(!b) value is -4 
(a << b) value is 16
(a >> b) value is 0
(a << b) value is 16
```

### 序列

Rust 提供了生成连续数值的简洁方式，例如：

- `1..5` 生成 1 到 4 的连续数字
- `1..=5` 生成 1 到 5 的连续数字

整数序列常用于循环：

```rust
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
```

```
1
2
3
4
5
```

序列只允许用于数字和字符类型。

**示例：** 字符类型序列

```rust
for i in 'a'..='z' {
    println!("{}",i);
}
```

### as 类型转换

### 有理数和复数

有理数和复数不在标准库中，`num` 库提供了这些功能。

使用 `num` 库：

1. 在 Cargo.toml 的 `[dependencies]` 下添加一行 `num = "0.4.1"`
2. 编写代码

```rust
use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);

    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
```

```
13.2 + 21i
```
