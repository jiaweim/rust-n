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

@author Jiawei Mao
***

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