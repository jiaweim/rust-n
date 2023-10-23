# 枚举

- [枚举](#枚举)
  - [简介](#简介)
  - [枚举值](#枚举值)
  - [同一化类型](#同一化类型)
  - [Option 枚举处理空值](#option-枚举处理空值)
  - [更多示例](#更多示例)

2023-10-23, 16:39
@author Jiawei Mao
****

## 简介

枚举通过列举可能的成员来定义类型，例如扑克牌花色：

```rust
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
```

## 枚举值

先创建两个 PokerSuit 实例：

```rust
let heart = PokerSuit::Hearts;
let diamond = PokerSuit::Diamonds;
```

接着定义一个函数使用它们：

```rust
fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}",card);
}
```

下面，我们想让扑克牌变得更实用，为每张牌赋予一个值 `A(1)`-`K(13)`，加上花色，就是一副扑克牌。

使用结构体实现方式：

```rust
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8
}

fn main() {
   let c1 = PokerCard {
       suit: PokerSuit::Clubs,
       value: 1,
   };
   let c2 = PokerCard {
       suit: PokerSuit::Diamonds,
       value: 12,
   };
}
```

这段代码可以很好完成我们所需的功能，不过还有更简洁的方式。

- 纯 enum 实现

```rust
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);
}
```

直接将数据与枚举成员关联，节省了许多代码。

- 同一枚举类型的不同成员可以持有不同的数据类型

例如，让某些花色打印 1-13 数字，另外的花色打印 A-K 的字符：

```rust
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds('A');
}
```

对这种情况，使用结构体实现就会很复杂。

- 来表标准库的实例

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

这里枚举成员包含的数据更复杂，使用结构体表示，分别使用 Ipv4Addr 和 Ipv6Addr 定义两种不同的 IP 数据。

- 任何类型的数据都可以放入枚举成员

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}
```

`Message` 包含 4 个成员：

- `Quit` 没有任何关联数据
- `Move` 包含一个匿名结构体
- `Write` 包含一个 `String`
- `ChangeColor` 包含 3 个 `i32`

如果用结构体的方式来定义这些信息：

```rust
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

每个结构体都有自己的类型，无法在需要同一类型的地方使用。从代码规范来讲，枚举的实现更简洁。

## 同一化类型

假设有一个 WEB 服务，需要接受用户的连接。假设连接有两种：`TcpStream` 和 `TlsStream`，但是我们希望对这两个连接的处理流程相同，即用同一个函数来处理。代码：

```rust
fn new (stream: TcpStream) {
  let mut s = stream;
  if tls {
    s = negotiate_tls(stream)
  }
  // websocket是一个WebSocket<TcpStream>或者
  //   WebSocket<native_tls::TlsStream<TcpStream>>类型
  websocket = WebSocket::from_raw_socket(
    stream, ......)
}
```

此时用枚举类型就很合适：

```rust
enum Websocket {
  Tcp(Websocket<TcpStream>),
  Tls(Websocket<native_tls::TlsStream<TcpStream>>),
}
```

## Option 枚举处理空值

Rust 用 `Option` 枚举表示可能存在空值的情况。

`Option` 枚举包含两个成员：

- `Some(T)`，表示含有值
- `None`，表示没有值

其中 `T` 是泛型参数。

示例：

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

如果使用 `None` 而不是 `Some`，必须告诉 Rust `Option<T>` 是什么类型，因为编译器无法通过 `None` 推断出类型。

使用 match 语句处理 Option enum：

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

## 更多示例

- 使用 as 将枚举值转换为整数类型

```rust
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // a enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
} 
```
