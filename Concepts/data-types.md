# 数据类型

- [数据类型](#数据类型)
  - [3. 复合类型](#3-复合类型)
    - [3.1. Tuple](#31-tuple)
    - [3.2. Array](#32-array)
      - [3.2.1. 访问数组元素](#321-访问数组元素)
      - [3.2.2. 无效数组访问](#322-无效数组访问)

Last updated: 2023-10-09, 21:40
@author Jiawei Mao
****

## 3. 复合类型

复合类型（compound type）将多个值组合为一种类型。Rust 有两种基本复合类型：Tuple 和 array。

- 数组（array）：如 `[1, 2, 3]`
- 元组（tuple）：如 `(1, true)`

### 3.1. Tuple

元组是一个可以包含各种类型值的组合。元组使用括号 `()` 来构造（construct），而每个元组自身又是一个类型标记为 `(T1, T2, ...)` 的值，其中 `T1`、`T2` 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。

```rust
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

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

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32, ));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)
}
```

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

