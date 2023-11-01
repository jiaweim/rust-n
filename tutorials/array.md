# 数组

- [数组](#数组)
  - [1. 简介](#1-简介)
  - [2. 创建数组](#2-创建数组)
  - [3. 访问数组元素](#3-访问数组元素)
    - [3.1. 越界访问](#31-越界访问)
    - [3.2. 非基础类型元素](#32-非基础类型元素)
  - [4. 数组切片](#4-数组切片)
  - [5. 示例](#5-示例)

2023-10-23, 17:01
@author Jiawei Mao
****

## 1. 简介

在 Rust 中有两种数组：

- 速度快但长度固定的 `array`
- 可动态增长但有性能损耗的 `Vector`

一般将 `array` 称为数组，`Vector` 称为动态数组。

数组的所有元素类型必须相同。

## 2. 创建数组

声明数组方式：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

由于类型大小固定，长度固定，因此数组 `array` 保存在栈上，性能较好。

动态数组 `Vector` 存储在堆上，因此长度可以动态改变。数组没有 `vector` 灵活，但性能更好，因此当元素个数已知时，推荐使用数组：

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

- 声明时指定类型和大小

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- 声明所有值相同的数组

`[类型; 长度]`

```rust
let a = [3; 5];
```

表示数组 `a` 大小为 5，所有元素都是 3，等价于 `let a = [3, 3, 3, 3, 3];`。

## 3. 访问数组元素

- 使用索引访问

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

### 3.1. 越界访问

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

该代码编译没问题，输入 0, 1, 2, 3, 4，程序会输出对应位置的元素值。如果超出该范围，例如 10，会出错：

```sh
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10',
src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

编译时 Rust 无法预知输入的索引值，所以这是一个运行时错误。

### 3.2. 非基础类型元素

- 创建非基础类型的数组

```rust
let array = [String::from("rust is good!"); 8];
println!("{:#?}", array);
```

编译错误：

```sh
error[E0277]: the trait bound `String: std::marker::Copy` is not satisfied
 --> src/main.rs:7:18
  |
7 |     let array = [String::from("rust is good!"); 8];
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` 
is not implemented for `String`
  |
  = note: the `Copy` trait is required because this value will be copied for each 
element of the array
```

基本类型在 Rust 中以 `Copy` 形式赋值，而 `String` 不支持。

- 合理方法

```rust
let array = [String::from("rust is good!"),String::from("rust is good!"),String::from("rust is good!")];

println!("{:#?}", array);
```

- 更好的做法

使用 `std::array::from_fn` 函数

```rust
let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

println!("{:#?}", array);
```

## 4. 数组切片

数组也支持切片：

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

这里 `slice` 的类型是 `&[i32]`，数组的类型是 `[i32;5]`。总结：

- 切片是对底层数组的引用，代价非常小
- 切片类型 `[T]` 大小不固定，而切片引用类型 `&[T]` 大小固定，Rust 许多地方都需要**固定大小的数据类型**，因此 `&[T]` 更有用，`&str` 字符串切片同理。

## 5. 示例

```rust
fn main() {
  // 编译器自动推导出one的类型
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
}
```

要点：

- 数组类型容易与数组切片混淆，`[T;n]` 是数组类型，而 `[T]` 是切片类型，因为切片是运行时的数据结构，它的长度无法再编译时知道，因此不能用 `[T;n]` 描述数组切片
- `[u8; 3]` 和 `[u8; 4]` 是不同类型，数组的长度也是类型的一部分
- 在实际开发中，使用最多的是数组切片 `[T]`，往往通过引用的方式使用 `&[T]`。

