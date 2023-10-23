# 元组

- [元组](#元组)
  - [简介](#简介)
  - [模式匹配解构元组](#模式匹配解构元组)
  - [访问元组](#访问元组)
  - [元组用作函数的参数和返回值](#元组用作函数的参数和返回值)
  - [示例](#示例)

2023-10-23, 14:19
@author Jiawei Mao
****

## 简介

复合类型（compound type）将多个值组合为一种类型。Rust 有两种基本复合类型：Tuple 和 array。

- 数组（array）：如 `[1, 2, 3]`
- 元组（tuple）：如 `(1, true)`

元组将多种类型的值组合到一起。元组长度固定，元组中元素的顺序固定。

创建元素语法：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

变量 `tup` 被绑定到一个元组值 `(500, 6.4, 1)`，该元素的类型是 `(i32, f64, u8)`。

## 模式匹配解构元组

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

这里首先创建一个元组，然后将其绑定到 tup 变量，接着使用 `let (x, y, z) = tup;` 进行模式匹配，元组中对应的值绑定到 x, y, z 上，这就是解构。

## 访问元组

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

和其它语言一样，元组索引从 0 开始。

## 元组用作函数的参数和返回值

函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。

```rust
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}
```

## 示例

```rust
// 在 “动手试一试” 的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印，最长 12
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)
}
```

没有值的元组有一个特殊名称：*unit*。该值及其类型都写为 `()`，指表达式不返回任何值。
