# 泛型

- [泛型](#泛型)
  - [简介](#简介)
  - [泛型详解](#泛型详解)
  - [结构体中使用泛型](#结构体中使用泛型)
  - [枚举中使用泛型](#枚举中使用泛型)
  - [方法中使用泛型](#方法中使用泛型)

@author Jiawei Mao
***

## 简介

泛型和特征是 Rust 中最重要的抽象类型。

我们在编程中，经常有这样的需求：用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，都支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数：

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

上述代码可以正常运行，但是很啰嗦，如果要支持更多的类型，会更繁琐。

泛型就是一种多态，为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能力。

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

将之前的代码改成上面这样，就是 Rust 泛型的初印象，这段代码虽然很简洁，但是不能通过编译，后面会进行详细讲解，现在只要对泛型有个大概的印象即可。

## 泛型详解

上面代码中的 `T` 是泛型参数。在 Rust 中，泛型参数的名称可以任意起，惯例首选 `T` ( T 是 type 的首字母)来作为首选。

使用泛型参数，需要在使用前对其进行声明：

```rust
fn largest<T>(list: &[T]) -> T {}
```

该泛型函数从列表中找出最大的值，列表中的元素类型为 `T`。首先 `largest<T>` 对泛型参数 `T` 进行了声明，然后才在函数参数中进行使用该泛型参数 `list: &[T]`（`&[T]` 是数组切片）。

函数 `largest` 有泛型类型 `T`，它有个参数 `list` ，其类型是元素为 `T` 的数组切片，最后，该函数返回值的类型也是 `T`。

下面是一个**错误**的泛型函数的实现：

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

因为 `T` 可以是任何类型，但不是所有类型都能进行比较，因此上面的错误中，编译器建议我们给 `T` 添加一个类型限制：使用 `std::cmp::PartialOrd` 特征（Trait）对 `T` 进行限制，下一节会详细介绍特征，现在只需理解，该特征让类型实现可比较的功能。

开头的 add 泛型函数同理，修改方式：

```rust
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

## 结构体中使用泛型

结构体中的字段类型也可以用泛型来定义，下面代码定义了一个坐标点 `Point` ，它可以存放任何类型的坐标值：

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

这里有两个要点：

- 提前声明，跟泛型函数定义类似，在使用泛型参数之前需要声明 `Point<T>`，接着就可以在结构体的字段类型中使用 `T` 来替代具体的类型
- x 和 y 是相同的类型

如果想让 x 和 y 既能类型相同，又能类型不同，就需要使用不同的泛型参数：

```rust
struct Point<T,U> {
    x: T,
    y: U,
}
fn main() {
    let p = Point{x: 1, y :1.1};
}
```

切记，所有的泛型参数都要提前声明： `Point<T,U>`。但是如果你的结构体变成这鬼样： `struct Woo<T,U,V,W,X>`，那么你需要考虑拆分这个结构体，减少泛型参数的个数和代码复杂度。

## 枚举中使用泛型

提到枚举类型，`Option` 永远是第一个应该被想起来的，在之前的章节中，它也多次出现：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` 是一个泛型枚举类型，第一个成员 `Some(T)` 存放了一个类型为 `T` 的值。得益于泛型，我们可以在任何一个需要返回值的函数中使用 `Option<T>` 作为返回值，返回一个任意类型的值 `Some(T)`，或者表示没有值的 `None`。

对于枚举而言，卧龙凤雏永远是绕不过去的存在：如果是 `Option` 是卧龙，那么 `Result` 就一定是凤雏，得两者可得天下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 和 `Option` 一样，主要用于函数返回值，与 `Option` 用于值的存在与否不同，`Result` 关注的主要是值的正确性。

- 如果函数正常运行，则返回 `Ok(T)`，`T` 是函数具体的返回值类型
- 如果函数异常运行，则返回 `Err(E)`，`E` 是错误类型

例如打开一个文件：如果成功打开文件，则返回 `Ok(std::fs::File)`，`T` 对应 `std::fs::File` 类型；而当打开文件时出现问题时，返回 `Err(std::io::Error)`，`E` 对应 `std::io::Error` 类型。

## 方法中使用泛型

方法上也可以使用泛型：

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

使用泛型参数前，依然需要提前声明 `impl<T>`，这样才能在 `Point<T>` 中使用它。需要注意的是，此时 `Point<T>` 不再是泛型声明，而是一个完整的结构体类型。

除了结构体中的泛型参数，还能在结构体的方法中定义额外的泛型参数，与泛型函数一样：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

