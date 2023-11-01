# 泛型

- [泛型](#泛型)
  - [1. 简介](#1-简介)
  - [2. 函数泛型](#2-函数泛型)
  - [3. 结构体泛型](#3-结构体泛型)
  - [4. 枚举泛型](#4-枚举泛型)
  - [5. 方法泛型](#5-方法泛型)
    - [5.1. 为特定泛型实现方法](#51-为特定泛型实现方法)
    - [5.2 方法的泛型参数](#52-方法的泛型参数)
  - [6. const 泛型（Rust 1.51+）](#6-const-泛型rust-151)
    - [6.1. const 泛型表达式](#61-const-泛型表达式)
  - [7. 泛型性能](#7-泛型性能)

2023-11-01, 16:28
update: 函数泛型
2023-10-26, 11:42
@author Jiawei Mao
****

## 1. 简介

泛型和 trait 是 Rust 最重要的抽象类型。

在编程中经常需要用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，都支持。在不支持泛型的编程语言中，需要为每种类型编写一个函数：

```rust
fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}

fn main() {
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));
}
```

上述代码可以正常运行，但是很啰嗦，如果要支持更多类型，会更繁琐。

泛型是一种多态，为程序员提供编程的便利，减少代码的臃肿，同时丰富语言本身的表达能力。

```rust
fn add<T>(a:T, b:T) -> T {
    a + b
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}
```

将之前的代码改成上面这样，就是 Rust 泛型的初印象，这段代码虽然简洁，但是不能通过编译，后面会进行详细解释，现在只要对泛型有个大概的印象即可。

## 2. 函数泛型

泛型参数名称可以采用任何标识符，首选 `T` ( type 首字母)。

使用泛型参数前需要声明，泛型参数声明位于函数名称与参数列表之间的 `<>` 中：

```rust
fn largest<T>(list: &[T]) -> T {}
```

该函数查找数组最大值：`largest<T>` 声明泛型参数 `T`，然后才在函数参数中使用该泛型参数，其参数 `list` 是元素类型为 `T` 的数组切片 `list: &[T]`，返回值类型也是 `T`。

下面是一个**错误**的泛型函数实现：

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

```sh
error[E0369]: binary operation `>` cannot be applied to type `T` // `>`操作符不能用于类型`T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T` // 考虑对T进行类型上的限制 :
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++
```

上面没有限制 `T` 的类型，但不是所有类型都支持比较。因此，上面的错误信息提示需要给 `T` 添加类型限制：使用 `std::cmp::PartialOrd` 特征对 `T` 进行限制，该特征让类型实现可比较的功能。

开头的 `add` 泛型函数同理，修改为：

```rust
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

## 3. 结构体泛型

结构体中的字段也可以使用泛型，下面定义坐标点 `Point` ，它可以存放任何类型的坐标值：

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

要点：

- 跟泛型函数定义类似，在使用泛型参数之前需要提前声明 `Point<T>`，接着就可以在结构体的字段类型中使用 `T` 来替代具体的类型
- `Point<T>` 只使用了一个泛型类型，所以 `x` 和 `y` 类型相同

如果不限制 `x` 和 `y` 类型相同，就需要加一个泛型参数：

```rust
struct Point<T,U> {
    x: T,
    y: U,
}
fn main() {
    let p = Point{x: 1, y :1.1};
}
```

切记，所有的泛型参数都要提前声明： `Point<T,U>`。

## 4. 枚举泛型

泛型枚举，以 `Option` 为例：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` 是一个拥有泛型 `T` 的枚举，它有两个成员：

- `Some(T)` 存放了一个类型为 `T` 的值
- `None` 表示没有值

`Option<T>` 常用作函数返回值。

枚举也可以拥有**多个泛型类型**，以 `Result` 为例：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 和 `Option` 一样，主要用于函数返回值，与 `Option` 用于值的存在与否不同，`Result` 关注的是值的正确性。

- 如果函数正常运行，返回 `Ok(T)`，`T` 是函数具体的返回值类型
- 如果函数异常运行，返回 `Err(E)`，`E` 是错误类型

例如打开一个文件：如果成功打开文件，则返回 `Ok(std::fs::File)`，`T` 对应 `std::fs::File` 类型；而当打开文件时出现问题时，返回 `Err(std::io::Error)`，`E` 对应 `std::io::Error` 类型。

## 5. 方法泛型

为结构体和枚举实现方法时，也可以使用泛型：

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

这里在 `Point<T>` 上定义了 `x` 方法来返回字段 `x` 中数据的引用。

这里必须在 `impl` 后声明 `T`，这样才能在 `Point<T>` 上实现的方法中使用 `T`。这里可以为泛型参数选择一个鱼结构体定义中声明的泛型参数不同的名称，不过惯例使用相同名称。需要注意的是，此时 `Point<T>` 不再是泛型声明，而是一个完整的结构体类型。

### 5.1. 为特定泛型实现方法

对 `Point<T>` 类型，不仅能定义基于 `T` 的方法，还能针对特定类型定义方法：

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

这里为 `Point<f32>` 类型实现 `distance_from_origin` 方法，其它 `T` 不是 `f32` 的 `Point<T>` 实例没有此方法。

### 5.2 方法的泛型参数

除了结构体中的泛型参数，在结构体的方法中可以定义额外的泛型参数

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

这里用 `self` 的 `Point` 类型的 `x` 值（`X1` 类型）和参数的 `Point` 类型的 `y` 值（`Y2` 类型）创建一个新的 `Point` 实例。

这里，`X1`, `Y1` 是定义在结构体 Point 上的泛型参数，`X2,Y2` 是定义在方法 `mixup` 上的泛型参数。一个是结构体泛型，一个是函数泛型。

## 6. const 泛型（Rust 1.51+）

在数组那节有提到一点：`[i32; 2]` 和 `[i32; 3]` 是不同的数组类型，比如:

```rust
fn display_array(arr: [i32; 3]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32;2] = [1,2];
    display_array(arr);
}
```

运行报错：

```sh
error[E0308]: mismatched types // 类型不匹配
  --> src/main.rs:10:19
   |
10 |     display_array(arr);
   |                   ^^^ expected an array with a fixed size of 3 elements, found one with 2 elements
                          // 期望一个长度为3的数组，却发现一个长度为2的
```

结合代码和报错可以看出，`[i32; 3]` 和 `[i32; 2]` 是两个不同的类型。

- 修改代码，让 `display_array` 能打印任意长度的 i32 数组

```rust
fn display_array(arr: &[i32]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32;2] = [1,2];
    display_array(&arr);
}
```

将参数设置为数组切片类型，然后传入 `arr` 的不可变引用即可。

- 继续将 `i32` 改成所有类型的数组

```rust
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32;2] = [1,2];
    display_array(&arr);
}
```

也不难，但是需要对 `T` 进行限制 `std::fmt::Debug`，表明 `T` 可以用在 `println!("{:?}", arr)` 中，因为 `{:?}` 格式化输出需要 `arr` 实现该特征。

通过引用可以解决任何类型数组的问题，但是如果引用不适用呢？

- `const` 泛型，针对值的泛型，可以处理数组长度问题

```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
```

这里，`T` 是基于类型的泛型参数，`N` 是基于值的泛型参数，用来替代数组的长度。

`N` 就是 const 泛型，语法为 `const N: usize`，表示 `const` 泛型 `N`，它基于的值类型是 `usize`。

在泛型参数之前，Rust 完全不适合复杂的矩阵运算，`const` 泛型解决了该问题。

### 6.1. const 泛型表达式

假设某段代码需要在内存很小的平台上运行，因此需要限制函数参数占用的内存。可以使用 `const` 泛型表达式实现：

```rust
// 目前只能在nightly版本下使用
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn something<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
{
    //
}

fn main() {
    something([0u8; 0]); // ok
    something([0u8; 512]); // ok
    something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}

// ---

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}
```

TODO

## 7. 泛型性能

在 Rust 中泛型是零成本的抽象，即在使用泛型时，不用担心性能问题。

但是任何选择权衡，既然获得了性能上的优势，那么失去了什么？Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度并增加了最终生成文件的大小。

具体来说，Rust 通过在编译时进行泛型代码的 **单态化**(monomorphization)来保证效率。单态化填充编译时使用的具体类型，将通用代码转换为特定代码。

编译器所做的工作正好与我们创建泛型函数的步骤相反，编译器寻找所有泛型代码被调用的位置并针对具体类型生成代码。

以 `Option` 为例：

```rust
let integer = Some(5);
let float = Some(5.0);
```

当 Rust 编译该代码时，会进行单态化。编译器读取传递给 `Option<T>` 的值，发现有两种 `Option<T>`，其中为 i32，一种为 f64.为此，编译器将泛型定义 `Option<T>` 展开为 `Option_i32` 和 `Option_f64`，接着将泛型定义替换为这两个具体的定义。

编译器生成的单态化版本的代码类似：

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。

