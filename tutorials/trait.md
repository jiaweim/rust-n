# trait

- [trait](#trait)
  - [1. 简介](#1-简介)
  - [2. 定义 trait](#2-定义-trait)
  - [3. 为类型实现 trait](#3-为类型实现-trait)
    - [3.1. trait 定义与实现的位置](#31-trait-定义与实现的位置)
    - [3.2. 默认实现](#32-默认实现)
  - [4. trait 作为参数](#4-trait-作为参数)
  - [5. trait 约束](#5-trait-约束)
    - [5.1. 多重约束](#51-多重约束)
    - [5.2. where 简化约束](#52-where-简化约束)
    - [5.3. 实现特定 trait 约束的方法](#53-实现特定-trait-约束的方法)
  - [6. 返回 impl trait](#6-返回-impl-trait)
  - [7. 修复 largest 函数](#7-修复-largest-函数)
  - [8. 通过 derive 派生 trait](#8-通过-derive-派生-trait)
  - [9. 更多示例](#9-更多示例)
    - [9.1. 自定义 + 操作](#91-自定义--操作)
    - [9.2. 自定义输出](#92-自定义输出)

2023-10-26, 14:58
@author Jiawei Mao
****

## 1. 简介

Rust 的 `trait` 和其它语言的接口类似。

之前已经见过许多 trait，例如 `#[derive(Debug)]` 在定义的类型上自动派生 `Debug` trait。再比如：

```rust
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}
```

通过 `std::ops::Add` trait 限制 `T`，只有 `T` 实现了 `std::ops::Add` 才能进行加法操作。

## 2. 定义 trait

定义 trait：定义一组实现某个目标所需的行为。

示例：

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

说明：

- 使用 `trait` 关键字声明 trait
- `Summary` 是特征名
- 在大括号中定义 trait 的所有方法，上例中为 `fn summarize(&self) -> String`
- trait 只定义方法签名，方法签名以 `;` 结尾，而不是一个 `{}` 

每一个实现该 trait 的类型都需要实现该 trait 的相应方法，编译器也会确保任何实现 `Summary` trait 的类型都拥有与这个签名完全一致的 `summarize` 方法。

## 3. 为类型实现 trait

为 `NewsArticle` 结构体实现 `Summary` trait，这里使用标题、作者和位置作为 `summarize` 的返回值：

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

实现 trait 的语法与为结构体、枚举实现方法很像： `impl Summary for NewsArticle`，然后在 `impl` 的花括号中实现该 trait 的具体方法。

接下来就可以在这个类型上调用 trait 的方法：

```rust
use rustings::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
```

```sh
1 new tweet: horse_ebooks: of course, as you probably already know, people
```

### 3.1. trait 定义与实现的位置

上面将 `Summary` 定义成了 `pub`。这样，其它人也可以引入 `Summary` trait 进行实现。

关于 trait 实现与定义的位置，有一条基本原则：如果想要为类型 `A` 实现 trait `T`，那么 `A` 或者 `T` 至少有一个在当前作用域中定义！例如:

- 可以为上面的 `NewsArticle` 类型实现标准库中的 `Display` trait，这是因为 `NewsArticle` 类型定义在当前作用域。
- 也可以在当前包中为 `String` 类型实现 `Summary` trait，因为 `Summary` 定义在当前作用域。

但是无法在当前作用域中，为 `String` 类型实现 `Display` trait，因为它俩都定义在标准库中，都不在当前作用域。

该规则被称为**孤儿规则**，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。

### 3.2. 默认实现

可以在 trait 中定义具有默认实现的方法，这样其它类型无需再实现该方法，或者也可以选择重载该方法：

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

上面为 `Summary` 定义了一个默认实现，下面测试该方法：

```rust
impl Summary for Post {}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

可以看到， `Post` 选择了默认实现，而 `Weibo` 重载了该方法，调用和输出如下：

```rust
println!("{}",post.summarize());
println!("{}",weibo.summarize());
```

```sh
(Read more...)
sunface发表了微博好像微博没Tweet好用
```

默认实现方法中允许调用相同特征中的其它方法，哪怕这些方法没有默认实现。例如，可以定义 `Summary` 特征，使其具有一个需要实现的 `summarize_author` 方法，然后定义一个 `summarize` 方法，此方法的默认实现调用`summarize_author` 方法：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

为了使用 `Summary` ，只需要实现 `summarize_author`：

```rust
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
println!("1 new weibo: {}", weibo.summarize());
```

`weibo.summarize()` 会先调用 `Summary` 特征默认实现的 `summarize` 方法，通过该方法进而调用 `Weibo` 为 `Summary` 实现的 `summarize_author` 方法，最终输出：`1 new weibo: (Read more from @horse_ebooks...)`。

## 4. trait 作为参数

使用 trait 作为函数参数：

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`impl Summary` 表示实现了 `Summary` 特征的 `item` 参数。

可以使用任何实现了 `Summary` 特征的类型作为该函数的参数，同时在函数体内可以调用该特征的方法，例如 `summarize` 方法。

## 5. trait 约束

`impl Trait` 这种语法非常直观，其实它是一种较长形式语法的语法糖。称为 trait 约束：

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

完整形式如上，`T: Summary` 被称为**trait 约束**。

在简单场景 `impl Trait` 就足够使用，但是对于复杂的场景，特征约束更灵活，语法表现能力更强，例如一个函数接受两个 `impl Summary` 的参数：

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

如果两个参数类型不同，上面的方法很好，只要这两个类型都实现 `Summary` 特征即可。但是如果想要强制函数的两个参数是同一类型呢？上面的语法就无法做到这种限制，此时只能使用特征约束来实现：

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

泛型类型 `T` 说明 `item1` 和 `item2` 类型必须相同，`T: Summary` 说明 `T` 必须实现 `Summary` 特征。

### 5.1. 多重约束

- 要求参数实现多个 trait 的语法糖形式

```rust
pub fn notify(item: &(impl Summary + Display)) {}
```

- trait 约束形式

```rust
pub fn notify<T: Summary + Display>(item: &T) {}
```

### 5.2. where 简化约束

当 trait 约束变多，函数的签名将变得很复杂：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

通过 `where` 可以简化函数签名：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

### 5.3. 实现特定 trait 约束的方法

特征约束，可以在指定类型 + 指定特征的条件下去实现方法，例如：

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

并不是所有 `Pair<T>` 结构体都可以调用 `cmp_display` 方法，只有 `T` 同时实现 `Display` 和 `PartialOrd` 的 `Pair<T>` 才拥有此方法。

也可以**有条件地实现特征**, 例如，标准库为任何实现了 `Display` 特征的类型实现了 `ToString` 特征：

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

我们可以对任何实现了 `Display` 特征的类型调用由 `ToString` 定义的 `to_string` 方法。例如，可
以将整型转换为对应的 `String` 值，因为整型实现了 `Display` ：

```rust
let s = 3.to_string();
```

## 6. 返回 impl trait

可以通过 `impl Trait` 来说明一个函数返回了一个类型，该类型实现了某个特征：

```rust
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```

因为 `Weibo` 实现了 `Summary`，因此这里可以用它来作为返回值。要注意的是，虽然我们知道这里是一个 `Weibo` 类型，但是对于 `returns_summarizable` 的调用者而言，他只知道返回了一个实现了 `Summary` 特征的对象，但是并不知道返回了一个 `Weibo` 类型。

这种 `impl Trait` 形式的返回值，在返回的真实类型非常复杂时很有用。例如，闭包和迭代器就很复杂，只有编译器才知道那玩意的真实类型，好在你可以用 `impl Iterator` 来告诉调用者，返回了一个迭代器，因为所有迭代器都会实现 `Iterator` 特征。

但是这种返回值方式有一个很大的限制：只能有一个具体的类型，例如：

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Weibo {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        }
    }
}
```

以上的代码就无法通过编译，因为它返回了两个不同的类型 `Post` 和 `Weibo` 。

```sh
`if` and `else` have incompatible types
expected struct `Post`, found struct `Weibo`
```

报错提示我们 if 和 else 返回了不同的类型。如果想要实现返回不同的类型，需要使用特征对象。

## 7. 修复 largest 函数

以下函数编译报错：

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
```

在 `largest` 函数中我们想要使用 `>` 比较两个 `T` 类型的值。该运算符是标准库中特征 `std::cmp::PartialOrd` 的一个默认方法，所以需要在 `T` 的特征约束中指定 `PartialOrd`。

由于 `PartialOrd` 位于 `prelude` 中所以并不需要通过 `std::cmp` 手动将其引入作用域。将 `largest` 的签名修改为：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

此时编译，出现新的错误：

```sh
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

错误的原因是 `T` 没有实现 `Copy` 特性，因此只能转移所有权，毕竟只有 `i32` 等基础类型实现了 `Copy` 特性，可以存储在栈上，而 `T` 可以指代任何类型（严格来说是实现了 `PartialOrd` 特征的所有类型）。

因此，为了让 T 拥有 Copy 特性，我们可以增加特征约束：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

如果并不希望限制 `largest` 函数只能用于实现了 `Copy` 特征的类型，可以在 `T` 的特征约束中指定 `Clone` 特征而不是 `Copy` 特征。使用 `clone` 意味着对类似 `String` 这样拥有堆上数据的类型，会潜在地分配更多堆上空间，而堆分配在涉及大量数据时可能会相当缓慢。

另一种 `largest` 的实现方式是返回在 list 中 `T` 值的引用。如果我们将函数返回值从 `T` 改为 `&T` 并改变函数体使其能够返回一个引用，我们将不需要任何 `Clone` 或 `Copy` 的特征约束而且也不会有任何的堆分配。

## 8. 通过 derive 派生 trait

`#[derive(Debug)]` 已经出现过很多次，这种是一种特征派生语法，被 `derive` 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

例如 `Debug` 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 `println!("{:?}", s)` 打印该结构体的对象。

再如 `Copy` 特征，它也有一套自动实现的默认代码，当标记到一个类型上时，可以让这个类型自动实现 `Copy` 特征，进而可以调用 `copy` 方法进行自我复制。

总之， `derive` 派生出来的是 Rust 默认给我们提供的特征，在开发过程中极大的简化了自己手动实现相应特征的需求，当然，如果你有特殊的需求，还可以自己手动重载该实现。

## 9. 更多示例

### 9.1. 自定义 + 操作

在 Rust 中除了数值类型，`String` 也可以做加法，因为 Rust 为 `String` 实现了 `std::ops::Add` 特征，同理，如果我们为自定义类型实现了该特征，那就可以自己实现 `Point1 + Point2` 的操作:

```rust
use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}
```

### 9.2. 自定义输出

在开发过程中，往往只要使用 `#[derive(Debug)]` 对自定义类型进行标注，即可打印输出：

```rust
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}
fn main() {
    let p = Point{x:3,y:3};
    println!("{:?}",p);
}
```

但是在实际项目中，往往需要对自定义类型进行格式化输出，以让用户更好地理解该类型，此时就要为自定义类型实现 `std::fmt::Display` 特征：

```rust
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl Display for FileState {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     match *self {
         FileState::Open => write!(f, "OPEN"),
         FileState::Closed => write!(f, "CLOSED"),
     }
   }
}

impl Display for File {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{} ({})>",
             self.name, self.state)
   }
}

impl File {
  fn new(name: &str) -> File {
    File {
        name: String::from(name),
        data: Vec::new(),
        state: FileState::Closed,
    }
  }
}

fn main() {
  let f6 = File::new("f6.txt");
  //...
  println!("{:?}", f6);
  println!("{}", f6);
}
```
