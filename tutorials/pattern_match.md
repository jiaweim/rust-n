# 模式匹配

- [模式匹配](#模式匹配)
  - [简介](#简介)
  - [match 匹配](#match-匹配)
    - [使用 match 表达式赋值](#使用-match-表达式赋值)
    - [模式绑定](#模式绑定)
    - [穷尽匹配](#穷尽匹配)
    - [通配符](#通配符)
  - [if let](#if-let)
  - [matches 宏](#matches-宏)
  - [变量遮蔽](#变量遮蔽)
  - [解构 Option](#解构-option)
  - [模式适用场景](#模式适用场景)
    - [match 分支](#match-分支)
    - [if let 分支](#if-let-分支)
    - [while let 条件循环](#while-let-条件循环)
    - [for 循环](#for-循环)
    - [let 语法](#let-语法)
    - [函数参数](#函数参数)
    - [let 和 if let](#let-和-if-let)
  - [全模式列表](#全模式列表)
    - [匹配字面量](#匹配字面量)
    - [匹配命名变量](#匹配命名变量)
    - [单分支多模式](#单分支多模式)
    - [..= 匹配值范围](#-匹配值范围)
    - [解构并分解值](#解构并分解值)
      - [解构结构体](#解构结构体)
      - [解构枚举](#解构枚举)
      - [解构嵌套的结构体和枚举](#解构嵌套的结构体和枚举)
      - [解构结构体和元组](#解构结构体和元组)
      - [解构数组](#解构数组)
    - [忽略模式中的值](#忽略模式中的值)
      - [使用 `_` 忽略整个值](#使用-_-忽略整个值)
      - [嵌套 `_` 忽略部分值](#嵌套-_-忽略部分值)
      - [使用下划线开头忽略未使用的变量](#使用下划线开头忽略未使用的变量)
      - [用 `..` 忽略剩余值](#用--忽略剩余值)
    - [匹配守卫提供的额外条件](#匹配守卫提供的额外条件)
    - [@ 绑定](#-绑定)

2023-10-25, 14:42
@author Jiawei Mao
****

## 简介

模式匹配常在函数式编程中用于复杂类型的结构。在 Rust 中，模式匹配常用 `match` 和 `if let`。

match 示例：

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

这里匹配 dire 对应的枚举类型，因此在 `match` 中用三个匹配分支来完全覆盖枚举变量 `Direction` 的所有成员类型，要点：

- `match` 匹配必须穷举所有可能，上面用 `_` 代表未列出的所有可能性
- `match` 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
- `X | Y`，类似逻辑运算符 `或` ，代表该分支可以匹配 X 也可以匹配 Y ，只要满足一个即可

`match` 跟其他语言中的 `switch` 非常像， `_` 类似于 `switch` 中的 `default`。

## match 匹配

`match` 语法：

```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```

`match` 将模式与 `target` 进行匹配。match 允许将一个值与一系列的模式比较，并根据匹配的模式执行对应代码。示例：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

`value_in_cents` 函数根据匹配的硬币，返回对应的美分数值。

- `match` 后是一个**表达式**，与 if 很像，但 if 后的表达式必须是布尔值，而 match 后的表达式返回值可以是任意值，只要能跟后面分支中的模式匹配起来即可。
- 后面是 match **分支**。分支由两部分组成：模式和针对该模式的处理代码。第一个分支的模式是 `Coin::Penny`，`=>` 将模式和执行代码分开。
- 不同分支使用逗号分隔。
- `match` 表达式将目标值 `coin` 按顺序与每个分支的模式进行比较，如果模式与目标值匹配，则执行模式之后的代码；如果模式与目标值不匹配，则继续执行下一个分支。
- 每个分支关联的代码是一个表达式，表达式的值将作为 `match` 表达式的返回值。如果分支有多行代码，则需要用 `{}` 包裹，且最后一行代码需要是一个表达式。

### 使用 match 表达式赋值

`match` 本身也是一个表达式，因此可以用它来赋值：

```rust
enum IpAddr {
   Ipv4,
   Ipv6
}

fn main() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}
```

这里匹配到 `_` 分支，所以将 `"::1"` 赋值给 `ip_str`。

### 模式绑定

模式匹配的一个重要功能是从模式中取出绑定的值，例如：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}
```

`Coin::Quarter` 成员存放了一个值：美国州名（1999-2008 年间，美国在 25 美分(Quarter)硬币的背后为 50 个州印刷了不同的标记，其它硬币没有这样的设计）。

接下来，我们希望在模式匹配中获取 25 美分硬币上刻的州名：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

这里在匹配 `Coin::Quarter(state)` 模式时，把它内部存储的值绑定到了 `state` 变量，因此 `state` 变量就是对应的 `UsState` 枚举类型。

再看一个更复杂的例子：

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```

```sh
$ cargo run
   Compiling world_hello v0.1.0 (/Users/sunfei/development/rust/world_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/world_hello`
Hello Rust
point from (0, 0) move to (1, 2)
change color into '(r:255, g:255, b:0)', 'b' has been ignored
```

### 穷尽匹配

match 的匹配必须穷尽所有情况。例如：

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}
```

这里没有处理 `Direction::West`，因此会报错：

```sh
error[E0004]: non-exhaustive patterns: `West` not covered // 非穷尽匹配，`West` 没有
被覆盖
  --> src/main.rs:10:11
   |
1  | / enum Direction {
2  | |     East,
3  | |     West,
   | |     ---- not covered
4  | |     North,
5  | |     South,
6  | | }
   | |_- `Direction` defined here
...
10 |       match dire {
   |             ^^^^ pattern `West` not covered // 模式 `West` 没有被覆盖
   |
   = help: ensure that all possible cases are being handled, possibly by adding 
wildcards or more match arms
   = note: the matched value is of type `Direction`
```

### 通配符

当我们不想在匹配时列出所有值的时候，可以使用通配符 `_`。

**示例：** u8 可以取 0 到 255 的所有值，但是我们只关心 1、3、5 和 7 这几个值，此时可以使用特殊的模式 `_` 替代

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

通过将 `_` 其放置于其他分支后， `_` 会匹配所有遗漏的值。 `()` 表示返回单元类型与所有分支返回值的类型相同，所以当匹配到 `_` 后，什么也不会发生。

除了 `_` 通配符，也可以用一个变量来承载其他情况。

```rust
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}
```

然而，在某些场景下，我们其实只关心某一个值是否存在，此时 `match` 就显得过于啰嗦。

## if let

有时只想一个模式的值，其它值直接忽略，此时用 `match` 处理很啰嗦：

```rust
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}
```

所以 Rust 引入了 if let 实现方式：

```rust
if let Some(3) = v {
    println!("three");
}
```

!!! note
    只匹配一个条件用 if let，否则用 match

## matches 宏

`matches!` 宏将一个表达式与模式进行匹配，返回 true 或 false。

假设有个动态数组，里面存有枚举类型：

```rust
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
}
```

此时如果需要对 v 进行过滤，只保留 `MyEnum::Foo` 元素，此时可以用 matches! 实现：

```rust
v.iter().filter(|x| matches!(x, MyEnum::Foo));
```

更多示例：

```rust
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));
```

## 变量遮蔽

无论 match 还是 if let 都是一个新的代码块，模式绑定引入新变量，如果变量与已有同名变量，就发生变量遮蔽：

```rust
fn main() {
   let age = Some(30);

   println!("在匹配前，age是{:?}",age);
   if let Some(age) = age {
       println!("匹配出来的age是{}",age);
   }
   println!("在匹配后，age是{:?}",age);
}
```

```sh
在匹配前，age是Some(30)
匹配出来的age是30
在匹配后，age是Some(30)
```

`if let` 语句中，`=` 右边 `Some(i32)` 的 `age` 被左边 `i32` 类型的 `age` 遮蔽了，该遮蔽持续到 `if let` 语句结束。

对 match 语句也是如此：

```rust
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   match age {
       Some(age) =>  println!("匹配出来的age是{}",age),
       _ => ()
   }
   println!("在匹配后，age是{:?}",age);
}
```

需要注意的是，match 中的变量遮蔽不容易看出，要格外小心。

## 解构 Option

Rust 用 `Option` 解决是否有值的问题。定义：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

使用 `Option<T>`，需要从 `Some` 中取出内部 `T` 值或处理没有值的情况。

**示例：** 获取一个 `Option<i32>`，如果含有值，将其 +1；如果没有值，返回 `None`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## 模式适用场景

模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据。

### match 分支

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

match 的每个分支就是一个**模式**，因为 match 是穷尽的，因此往往需要一个特殊的模式 `_` 来匹配剩余情况：

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}
```

### if let 分支

`if let` 用于匹配一个模式，而忽略剩下模式：

```rust
if let PATTERN = SOME_VALUE {

}
```

### while let 条件循环

`while let` 只要模式匹配就一直执行 `while` 循环。

**示例：**

```rust
// Vec是动态数组
let mut stack = Vec::new();

// 向数组尾部插入元素
stack.push(1);
stack.push(2);
stack.push(3);

// stack.pop 从数组尾部弹出元素
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

```sh
3
2
1
```

`pop` 取出动态数组的最后一个元素并返回 `Some(value)`，如果动态数组是空的，返回 `None`。对 `while` 来说，只要 `pop` 返回 `Some` 就一直循环，一旦返回 `None`，`while` 循环停止。

这里也可以用 `loop` + `if let` 或者 `match` 实现该功能，但语法更啰嗦。

### for 循环

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

这里用 `enumerate` 产生一个迭代器，该迭代器每次返回一个 `(索引, 值)` 形式的元组，然后用 `(index,value)` 来匹配。

### let 语法

```rust
let PATTERN = EXPRESSION;
```

这也是一种模式匹配：

```rust
let x = 5;
```

其中，`x` 是一种模式绑定，表示将匹配的值绑定到变量 `x` 上。因此变量名也是一种模式。

```rust
let (x, y, z) = (1, 2, 3);
```

上面将一个元组与模式进行匹配(**模式和值的类型必需相同**)，然后把 1, 2, 3 分别绑定到 x, y, z 上。

模式匹配要求两边的类型相同，否则报错：

```rust
let (x, y) = (1, 2, 3);
```

```sh
error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
4 | let (x, y) = (1, 2, 3);
  |     ^^^^^^   --------- this expression has type `({integer}, {integer}, 
{integer})`
  |     |
  |     expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
```

对元组而言，元素个数也是类型的一部分。

### 函数参数

函数参数也是模式：

```rust
fn foo(x: i32) {
    // 代码
}
```

`x` 是一个模式，它可以在参数中匹配元组：

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

`&(3, 5)` 会匹配模式 `&(x, y)` ，因此 `x` 得到了 `3` ， `y` 得到了 `5`。

### let 和 if let

以下代码，编译器报错：

```rust
let Some(x) = some_option_value;
```

因为右边的值可能不为 Some，而是 None，此时不能匹配。

类似 let，for 和 match 都要求完全覆盖匹配，才能通过编译。

但是 `if let` 可以：

```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

因为 if let 允许匹配一种模式，而忽略其它模式。

## 全模式列表

有许多不同类型的模式，下面将这些模式语法都列出来。

### 匹配字面量

```rust
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

因为 `x` 是 1，所以会输出 `"one"`。

### 匹配命名变量

在匹配命名变量时会遇到变量遮蔽：

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

说明：

- 第一个分支的模式不匹配 `x` 值，代码继续执行；
- 第二个分支的模式引入新变量 `y`，它匹配任何 `Some` 中的值。因为这里的 `y` 在 `match` 表达式的作用域中，是一个新变量。这个新 `y` 绑定会匹配任何 Some 中的值，在这里是 x 中的值，因此这个 `y` 绑定了 x 中 Some 内部值。
- 如果 x 的值为 `None`，前两个分支的模式不会匹配，所以会匹配模式 `_`。这个分支的模式没有引入变量 `x`，所以此时表达式 x 就是外部没有被遮蔽的 `x`。


如果不想引入变量遮蔽，可以使用其它变量名而非 `y`。

### 单分支多模式

在 match 表达式中，可以使用 `|` 匹配多个模式，代表**或**。例如：

```rust
let x = 1;
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

```sh
one or two
```

### ..= 匹配值范围

序列语法不仅可以用于循环，还能用于匹配模式。

`..=` 允许匹配一个闭区间序列内的值。当模式匹配任何在此序列内的值，该分支会执行：

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

如果 `x` 是1、2、3、4、5，第一个分支匹配，这比使用 `|` 更方便。

序列只允许用于数字或字符类型。

- 使用字符类型序列

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

```sh
early ASCII letter
```

### 解构并分解值

可以使用模式来结构结构体、枚举、元素、数组和引用。

#### 解构结构体

用 `let` 解构带两个字段 x 和 y 的结构体 Point:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

这里创建变量 `a` 和 `b` 来匹配结构体 `p` 中的 `x` 和 `y` 字段，同时展示模式中的变量名不必与结构体中的字段名一致。不过通常建议变量名与字段名一致，便于理解。

因为变量名匹配字段名很常见，如果变量名与字段名相同，可以简化语法：

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

- 使用字面值作为结构体模式的一部分进行解构

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

- 首先 `match` 第一个分支，指定匹配 `y` 为 `0` 的 `Point`； 
- 然后第二个分支在第一个分支之后，匹配 `y` 不为 `0`，`x` 为 `0` 的 Point ; 
- 最后一个分支匹配 x 不为 0， y 也不为 0 的 Point 。

在这个例子中，值 p 因为其 x 包含 0 而匹配第二个分支，因此会打印出 "On the y axis at 7"。

#### 解构枚举

使用 match 解构枚举的内部值：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}
```

```sh
Change the color to red 0, green 160, and blue 255
```

模式匹配一定要类型相同，因此匹配 `Message::Move{1,2}` 这样的枚举值，必须要用 `Message::Move{x,y}` 这样的同类型模式才行。

像 `Message::Quit` 这样没有任何数据的枚举成员，不能进一步解构。只能匹配其字面值 `Message::Quit`，因此模式中没有任何变量。

#### 解构嵌套的结构体和枚举

match 也可以匹配嵌套项。示例：

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}
```

match 第一个分支的模式匹配 `Message::ChangeColor` 枚举成员，该枚举成员又包含一个 `Color::Rgb` 枚举成员，最终绑定 3 个内部 `i32` 值。

#### 解构结构体和元组

可以用复杂的方式来混合、匹配和嵌套解构模式。

**示例：** 结构体和元组嵌套在元组中，将所有原始类型解构出来

```rust
struct Point {
     x: i32,
     y: i32,
 }

let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

#### 解构数组

- 定长数组

```rust
let arr: [u16; 2] = [114, 514];
let [x, y] = arr;

assert_eq!(x, 114);
assert_eq!(y, 514);
```

- 不定长数组

```rust
let arr: &[u16] = &[114, 514];

if let [x, ..] = arr {
    assert_eq!(x, &114);
}

if let &[.., y] = arr {
    assert_eq!(y, 514);
}

let arr: &[u16] = &[];

assert!(matches!(arr, [..]));
assert!(!matches!(arr, [x, ..]));
```

### 忽略模式中的值

#### 使用 `_` 忽略整个值

用在函数参数中：

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

这段代码会忽略第一个参数传入的值 `3`，并打印 `This code only uses the y parameter: 4`。

该功能可以用在未完全实现的函数中。

#### 嵌套 `_` 忽略部分值

在模式内部使用 `_` 忽略部分值：

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```

```sh
Can't overwrite an existing customized value
setting is Some(5)
```

第一个匹配分支，不关心里面的值，只关心两个元素的类型，因此直接忽略 `Some` 中的值。

剩下的形式，如 `(Some(_),None)`，`(None, Some(_))`, `(None,None)` 都由第二个分支 `_` 匹配。

还可以在一个模式中的多处使用下划线来忽略特定值。

**示例：** 忽略一个 5 元元组中第二个和第四个值

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

#### 使用下划线开头忽略未使用的变量

Rust 对未使用变量会给出警告，如果希望 Rust 不要警告未使用的变量，此时可以用下划线作为变量名的开头：

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

这里只警告说未使用变量 y，x 则没有警告。

注意, 只使用 `_` 和使用以下划线开头的名称不同：比如 `_x` 仍会将值绑定到变量，而 `_` 则完全不会绑定。

```rust
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

`s` 是一个拥有所有权的动态字符串，上面代码会报错，因为 `s` 的值被转移给 `_s`，在 println! 中再次使用 `s` 报错：

```sh
error[E0382]: borrow of partially moved value: `s`
 --> src/main.rs:8:22
  |
4 |     if let Some(_s) = s {
  |                 -- value partially moved here
...
8 |     println!("{:?}", s);
  |                      ^ value borrowed here after partial move
```

只使用下户线，则不会绑定值：

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

#### 用 `..` 忽略剩余值

`..` 模式忽略模式中剩余的没有显式匹配的值。

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

这里列出 `x` 值，然后用 `..` 模式来忽略其它字段，该语法比一一列出其它字段，然后用 `_` 忽略简洁。

- 还可以用 `..` 忽略元组中间的某些值

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

这里用 `first` 和 `last` 来匹配第一个和最后一个值。 `..` 将匹配并忽略中间的所有值。

使用 `..` 必须是无歧义的。如果期望匹配和忽略的值不明确，Rust 会报错。

**示例：** 带有歧义的 `..`

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

```sh
error: `..` can only be used once per tuple pattern // 每个元组模式只能使用一个 `..`
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here // 上一次使用在这里

error: could not compile `world_hello` due to previous error              ^^
```

Rust 无法判断 `second` 应该匹配 `numbers` 中的第几个元素，因此这里使用两个 `..` 模式有歧义。

### 匹配守卫提供的额外条件

**匹配守卫**（match guard）是一个位于 `match` 分支模式之后的额外 `if` 条件，它能为分支模式提供进一步的匹配条件。

这个条件可以使用模式中创建的变量：

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

```sh
less than five: 4
```

`num` 与模式中第一个分支匹配时，`Some(4)` 与 `Some(x)` 匹配上，接着匹配守卫检查 `x` 值是否小于 `5`，`4` 小于 `5`，所以第一个分支被选择。

相反，如果 `num` 为 `Some(10)`，因为 10 不小于 5 ，所以第一个分支的匹配守卫为假。Rust 前往第二个分支，这里没有匹配守卫，所以会匹配任何 `Some` 成员。

模式中无法提供类如 `if x < 5` 的表达能力，但可以通过**匹配守卫**实现。

可以使用匹配守卫来解决模式中变量覆盖的问题，那里 `match` 表达式的模式中新建了一个变量而不是使用 `match` 之外的同名变量。内部变量覆盖了外部变量，意味着此时不能够使用外部变量的值，下面代码展示了如何使用匹配守卫修复这个问题。

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
```

```sh
Default case, x = Some(5)
at the end: x = Some(5), y = 10
```

现在第二个匹配分支中的模式不会引入一个覆盖外部 `y` 的新变量 `y`，因此可以在匹配守卫中使用外部 `y` 。相比指定会覆盖外部 y 的模式 `Some(y)`，这里指定为 `Some(n)`。此新建的变量 n 并没有覆盖任何值，因为 match 外部没有变量 `n`。

匹配守卫 `if n == y` 并不是一个模式，所以没有引入新变量。这个 `y` 是外部 `y` 而不是新的覆盖变量 `y` ，这样就可以通过比较 n 和 y 来寻找一个与外部 y 相同的值。

- 可以在匹配守卫中使用 **或** 运算符 `|` 来指定多个模式

**示例：** 匹配 x 值为 4 、 5 或 6 同时 y 为 true 的情况。

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

上式优先级为：

```rust
(4 | 5 | 6) if y => ...
```

### @ 绑定

`@` 运算符将一个字段绑定另外一个变量。

**示例：** 测试 `Message::Hello` 的 `id` 字段是否在 `3..=7` 范围内，同时将其值绑定到 `id_variable` 变量中以便此分支中相关的代码可以使用它。也可以将 `id_variable` 命名为 `id`，即与字段同名。

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

```sh
Found an id in range: 5
```

在 `3..=7` 前面指定 `id_variable @` 捕获匹配此范围的值并将该值绑定到变量 `id_variable`。

第二个分支只在模式中指定了范围， `id` 字段的值可以是 10、11 或 12 ，不过这个模式的代码不能使用 `id` 字段中的值，因为没有将 id 值保存进一个变量。

最后一个分支指定了一个没有范围的变量，此时确实拥有可以用于分支代码的变量 id ，因为这里使用了结构体字段简写语法。不过此分支中没有像头两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。

当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。

- 使用 @ 绑定新变量并解构（Rust 1.56+）

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);


    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
```

- @ 新特性（Rust 1.53+）

```rust
fn main() {
    match 1 {
        num @ 1 | 2 => {
            println!("{}", num);
        }
        _ => {}
    }
}
```

编译不通过，因为 `num` 只绑定了模式 `1`，没有绑定所有的模式。在 Rust 1.53+ 可以按如下方式修改：

```rust
num @ (1 | 2)
```

