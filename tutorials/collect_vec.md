# 动态数组

- [动态数组](#动态数组)
  - [简介](#简介)
  - [创建 Vec](#创建-vec)
    - [Vec::new](#vecnew)
    - [vec!](#vec)
  - [添加元素](#添加元素)
  - [作用域](#作用域)
  - [从 Vector 中读取元素](#从-vector-中读取元素)
  - [同时借用多个元素](#同时借用多个元素)
  - [遍历元素](#遍历元素)
  - [存储不同类型的元素](#存储不同类型的元素)
  - [排序](#排序)
    - [整数数组排序](#整数数组排序)
    - [浮点数组排序](#浮点数组排序)
    - [结构体数组排序](#结构体数组排序)

2023-10-26, 21:34
@author Jiawei Mao
****

## 简介

动态数组 `Vec<T>`，存储相同类型元素。

## 创建 Vec

### Vec::new

- 指定类型

```rust
let v: Vec<i32> = Vec::new();
```

这里显式指定 `v` 的类型为 `Vec<i32>`，因为编译器无法从 `Vec::new()` 无法推导出 `v` 的具体类型。

- 不指定类型

```rust
let mut v = Vec::new();
v.push(1);
```

此时无需指定 `v` 的类型，编译器通过 `v.push(1)` 可以推测出 `v` 中的元素类型是 `i32`，从而推导出 `v` 的类型是 `Vec<i32>` 。

!!! note
    如果预先知道元素个数，可以使用 `Vec::with_capacity(capacity)` 创建动态数组，这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能。

### vec!

使用宏 `vec!` 创建数组，与 `Vec::new` 不同，前者能在创建时初始化值：

```rust
let v = vec![1, 2, 3];
```

此处 v 也无需标注类型，编译器可以根据内部元素自动推导出 `v` 的类型是 `Vec<i32>`。

## 添加元素

使用 `push` 向数组尾部添加元素：

```rust
let mut v = Vec::new();
v.push(1);
```

与其它类型一样，必须将 `v` 声明为 mut 才能修改。

## 作用域

跟结构体一样，`Vector` 超出作用域后会被删除：

```rust
{
    let v = vec![1, 2, 3];

    // ...
} // <- v超出作用域并在此处被删除
```

当 `Vector` 被删除后，它内部存储的所有内容也会随之被删除。目前来看，这种解决方案简单直白，但是当 Vector 中的元素被引用后，事情可能会没那么简单。

## 从 Vector 中读取元素

读取指定位置的元素有两种方式：

- 通过下标索引访问。
- 使用 get 方法。

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {third}"),
    None => println!("去你的第三个元素，根本没有！"),
}
```

和其它语言一样，Vec 索引从 0 开始：

- `&v[2]` 表示借用 `v` 的第三个元素，获得该元素的引用。
- `v.get(2)` 也是访问第三个元素，但是返回 `Option<&T>`，因此还需要 match 来匹配解构出具体的值。

这两种方式都能读取指定位置的元素，但 `&v[index]` 越界直接报错，而 `v.get(index)` 不会。例如：

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

这里 `&v[100]` 直接报错，`v.get(100)` 则返回 `None`。即使用 `v.get`更安全。

## 同时借用多个元素

例如：

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

首先，`let first = &v[0]` 不可变借用第一个元素，`v.push` 可变借用，如果 first 在 `v.push` 不再使用，该代码可以编译。

可是上面的代码在 v.push 之后又用了 first，毫无疑问编译器会报错：

```sh
$ cargo run
Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable 无法对v进行可变借用，因此之前已经进行了不可变借用
--> src/main.rs:6:5
|
4 |     let first = &v[0];
|                  - immutable borrow occurs here // 不可变借用发生在此处
5 |
6 |     v.push(6);
|     ^^^^^^^^^ mutable borrow occurs here // 可变借用发生在此处
7 |
8 |     println!("The first element is: {}", first);
|                                          ----- immutable borrow later used here // 不可变借用在这里被使用

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```

按理来说，这两个引用互相不影响：一个是查询元素，一个是在数组尾部插入元素，互不干扰，编译器为何要这么严格？

**原因在于**：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。此时，之前的引用会指向一块无效内存。

## 遍历元素

遍历比用索引的方式更安全、高效：

```rust
let v = vec![1, 2, 3];
for i in &v {
    println!("{i}");
}
```

可以在迭代过程中修改 Vector 的元素：

```rust
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 10
}
```

## 存储不同类型的元素

数组的元素类型相同，如果想存储不同类型，可以通过使用**枚举**和**特征**间接实现。

- 通过枚举实现

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}
fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}
```

数组 `v` 中存储了两种不同的 ip 地址，但是它们都是 `IpAddr` 枚举类型，因此可以存储在数组中。

- 通过特征实现

```rust
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
```

比枚举实现稍微复杂一点，这里 `V4` 和 `V6` 都实现了特征 `IpAddr`，然后将它俩的实例用 `Box::new` 包裹后，存放到数组 `v`，需要注意的是，这里必须手动指定类型 `Vec<Box<dyn IpAddr>>`，表示数组 `v` 存储的是特征 `IpAddr` 的对象。

在实际使用场景中，特征对象数组要比枚举数组常见很多，主要原因在于特征对象非常灵活，而编译器对枚举的限制较多，且无法动态增加类型。

## 排序

rust 提供了两种排序算法：

- 稳定的排序 `sort` 和 `sort_by`
- 非稳定排序 `sort_unstable` 和 `sort_unstable_by`

这个 `非稳定` 指在排序过程中对相等元素的处理方式。`稳定` 排序算法不会相等元素重新排序。而 `不稳定` 算法不能保证这点。

总体而言，`非稳定` 排序算法的速度会优于 `稳定` 排序算法，同时，`稳定` 排序还会额外分配原数组一半的空间。

### 整数数组排序

**示例：**

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

### 浮点数组排序

使用和整数一样的方法排序：

```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```

结果报错了：

```sh
error[E0277]: the trait bound `f32: Ord` is not satisfied
    --> src/main.rs:29:13
     |
29   |         vec.sort_unstable();
     |             ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `f32`
     |
     = help: the following other types implement trait `Ord`:
               i128
               i16
               i32
               i64
               i8
               isize
               u128
               u16
             and 4 others
note: required by a bound in `core::slice::<impl [T]>::sort_unstable`
    --> /home/keijack/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2635:12
     |
2635 |         T: Ord,
     |            ^^^ required by this bound in `core::slice::<impl [T]>::sort_unstable`

For more information about this error, try `rustc --explain E0277`.
```

原来，浮点数中因为存在一个 `NAN` 值，该值无法与其它浮点数进行对比，因此，浮点数类型并没有实现全数值可比较 `Ord` 的特性，而是实现了部分可比较的特性 `PartialOrd` 。

因此，如果我们确定浮点数数组中没有 `NAN` 值，那么可以使用 `partial_cmp` 来比较大小。

```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```

### 结构体数组排序

对结构体也可以使用自定义对比函数的方式来进行排序：

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);
}
```

```sh
[Person { name: "Al", age: 60 }, Person { name: "Zoe", age: 25 }, Person { name: "John", age: 1 }]
```

排序需要实现 `Ord` 特性，如果结构体实现了该特性，是否就不需要自定义对比函数？

是，但不完全是，实现 `Ord` 需要我们实现 `Ord` 、 `Eq` 、 `PartialEq` 、 `PartialOrd` 这些属性。好消息是，你可以 `derive` 这些属性：

```rust
#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}
```

```sh
[Person { name: "Al", age: 30 }, Person { name: "Al", age: 60 }, Person { name: "John", age: 1 }, Person { name: "John", age: 25 }, Person { name: "Zoe", age: 25 }]
```

`derive` `Ord` 相关特性，需要确保结构体中所有属性均实现了 `Ord` 相关特性，否则会发生编译错误。 `derive` 的默认实现会依据属性的顺序依次进行比较，如上述例子中，当 `Person` 的 `name` 值相同，则会使用 `age` 进行比较。
