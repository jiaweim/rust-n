# 所有权的概念

- [所有权的概念](#所有权的概念)
  - [简介](#简介)
  - [堆和栈](#堆和栈)
  - [Ownership 规则](#ownership-规则)
  - [变量 scope](#变量-scope)
  - [String 类型](#string-类型)
  - [内存和分配](#内存和分配)
    - [变量和数据交互：Move](#变量和数据交互move)
    - [变量和数据交互：Clone](#变量和数据交互clone)
    - [Stack 数据：Copy](#stack-数据copy)
  - [Ownership 和函数](#ownership-和函数)
  - [返回值和 Scope](#返回值和-scope)

Last updated: 2023-10-10, 19:49
@author Jiawei Mao
****

## 简介

所有权（ownership）是 Rust 的独特特性。它使 Rust 在没有垃圾收集器的情况下保证内存安全，因此理解 ownership 很重要。下面讨论 ownership 的几个概念：

- borrowing
- slices
- Rust 如何在内存中布局数据

*Ownership* 是一组控制 Rust 管理内存的规则。

所有程序在运行时都必须管理它们使用计算机内存的方式：

- 一些语言有垃圾回收，在程序运行时定期检查不再使用的内存。
- 有些语言需要程序员显式分配和释放内存。
- Rust 使用了一种新的方式：使用 ownership 管理内存，ownership 包含一组编译器检查的规则。违反任何规则，都无法编译成功。ownership 的所有特性都不会减慢程序的速度。

## 堆和栈

许多编程语言不需要考虑堆和栈，但像 Rust 这样的系统编程语言，数据在堆还是栈上会影响语言的行为。

堆和栈都是代码运行时可以使用的内存部分，但它们的结构不同。栈按获取值的顺序存储值，按相反的顺序删除值，即后进先出。存储在 stack 上的数据必须具有已知的固定大小，在编译时大小未知或大小可变的数据必须存储在 heap 中。

heap 相对无序：当你将数据放到 heap 上，请求一定大小的内存。内存 allocator 在 heap 中找到一个足够大的空间，将其标记为正在使用，并返回一个指针，指向该位置的地址，这个过程称为 allocating（将数据 压入 stack 不叫 allocating）。因为指向 heap 的指针是已知的固定大小，因此可以将指针存储在 stack 上，但需要实际数据时，必须遵循指针。

- 压入 stack 比在 heap 上 allocating 要快，因为 stack 不需要寻址，位置总是 stack 顶部。而在 heap 中，allocator 需要先找一个足够大的空间。
- heap 中的数据访问也比 stack 上慢，因为 heap 中必须根据指针到达数据位置。


当代码调用函数时，传递给函数的值（包括可能指向 heap 中数据的指针）和函数的局部变量被压入 stack。函数结束时，这些值从 stack 中弹出。

跟踪代码中哪部分正在使用 heap 中哪些数据，最小化 heap 上重复数据，以及清理 heap 上未使用的数据，这些都是 ownership 可以解决的问题。理解 ownership 后，就不需要经常考虑 stack 和 heap，但是一定要知道 ownership 的主要目的是管理 heap 数据。

## Ownership 规则

Ownership 规则：

- Rust 中每个值都有一个 owner。
- 一次只能有一个 owner。
- 当 owner 超出 scope，对应值被删除。

## 变量 scope

变量的作用域（scope）是 ownership 的一种。scope 表示有效的范围。例如，定义一个变量：

```rust
let s = "hello";
```

变量 `s` 引用一个字符串字面量，从声明的地方开始，到当期那 scope 结尾该变量有效。如下：

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

这里有两个重要节点：

- 当 `s` 进入 scope，它生效
- 在超出 scope 之前 `s` 一直有效

Rust 作用域和其它编程语言一样。

## String 类型

为了解释 ownership 的规则，下面介绍字符串，一个比基本类型更复杂的数据类型。

前面介绍的类型大小是已知的，可以存储在 stack 中，当它们的 scope 结束时从 stack 弹出，如果其它代码需要在不同 scope 使用相同的值，也可以快速复制一个新的、独立的实例。

我们主要关心 heap 上的数据，并探索 Rust 如何清理 heap 上的数据，`String` 类型是一个很好的例子。对 String 的 ownership 相关概念，也适用于其他复杂类型。

`String` 的数据存储在 heap 上，因此能够存储在编译时未知的大量文本。可以使用 `String.from` 函数从字符串字面量创建 `String`：

```rust
let s = String::from("hello");
```

这类字符不像字符串字面量，可以被修改：

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

那么，这里有什么不同？为什么可以修改 String，但字面量不行。关键不同在于这两种类型处理内存的方式。

## 内存和分配

对字符串字面量，在编译时就知道内容，即直接硬编码到最终的可执行文件中。

`String` 类型为了支持可变的、可增长的，需要在 heap 上分配一定数量的内存，在编译时未知。因此：

- 必须在运行时从 allocator 请求内存
- 使用完 `String`，需要一种方法将这些内存还给 allocator

第一部分由我们自己完成，当我们调用 `String::from`，其实现会请求所需的内存，在编程语言中普遍如此。

第二部分就不同了。在具有垃圾回收器（GC）的语言中，GC 会跟踪并清理不再使用的内存；在没有 GC 的语言中，需要程序员手动释放内存，而正确释放内存很困难。

Rust 采用不同的方式：当变量超出 scope，就自动返还内存。例如:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                         // this scope is now over, and s is no longer valid
```

当 `s` 超出 scope，是释放 `String` 占有的内存给 allocator 最佳时刻。当变量超出 scope，Rust 会自动调用一个特殊的 `drop` 函数。

!!! note
    在 C++ 中，这种在生命周期结束释放资源的模式称为 Resource Acquisition Is Initialization (RAII)。如果使用过 RAII，就会很熟悉 drop 函数。

这种模式对 Rust 代码的编写方式影响很大。此时看起来很简单，但是在更复杂的情况，当我们想让多个变量使用分配到  heap 上的数据时，代码的行为可能会出乎意料。

### 变量和数据交互：Move

在 Rust 中，多个变量可以用不同的方式与相同的数据交互。例如：

```rust
let x = 5;
let y = x;
```

这里，将 `5` 绑定到 x；然后复制 x 的值，并将其绑定到 `y`。此时变量 x 和 y 的值都是 5。因为整数是固定大小、已知的简单值，因此这两个 `5` 被压入 stack。

再来看 String 版本：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

虽然代码看起来一样，但行为完全不同。如下图所示：

@import "images/trpl04-01.png" {width="360px" title=""}

> 图 1：字符串在内存中的表示

String 由三部分组成：

- 指向内存的指针
- length
- capacity

这部分信息存储在 stack 上。右边内容保存在 heap。

length 是 String 单当前使用的内存（bytes）。capacity 是 `String` 从 allocator 接收到的内存总量。

将 s1 赋值给 s2 时，String 的数据被复制，即复制 stack 上的指针、length 和 capacity，不会复制指针指向的 heap 上的数据。此时内存中的数据如图所示：

@import "images/trpl04-02.png" {width="360px" title=""}

> 图 2：字符 `s1` 的副本 `s2` 在内存中的表示。

如果同时复制 heap 上的数据，如下所示：

@import "images/trpl04-03.png" {width="360px" title=""}

> 图 3：如果同时复制 heap 数据，`s2 = s1` 的内存表示。

如果 heap 上的数据很大，复制 heap 数据会使 `s2 = s1` 操作在运行时性能就会很差。

前面说过，当变量超出 scope 时，Rust 会自动调用 `drop` 函数清理该变量指向的 heap 内存。如图 2 所示，两个指针指向相同位置。此时有个问题：当 s2 和 s1 超出 scope，它们都将尝试释放相同的内存，这被称为 *double free* error。两次释放内存可能导致内存损坏，从而导致安全漏洞。

为了确保内存安全，在 `let s2 = s1;` 后，Rust 认为 s1 失效。因此当 s1 超出 scope，Rust 不需要释放任何东西。下面看看创建 s2 后，继续使用 s1 会有什么效果：

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

运行会报错：

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

Rust 这种行为和浅复制有点类似，但是 Rust 直接使第一个变量失效，该行为在 Rust 中称为移动（move）。对该示例，我们可以说 s1 移到了 s2.因此，实际的情况是：

@import "images/trpl04-04.png" {width="360px" title=""}

> 图 4：s1 失效后的内存表示。

这样问题就解决了，当 s2 超出 scope，它会单独释放内存。

该设计也说明 Rust 不会自动创建数据的深副本。因此，在运行时任何自动复制都是廉价高效的。

### 变量和数据交互：Clone

如果需要深度复制字符串的 heap 数据，而不仅仅是 stack 数据，可以使用 `clone` 方法。

示例：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

这样会显式产生图 3 的效果，即复制 heap 数据。

深度复制操作比较昂贵。

### Stack 数据：Copy

对整数，如下所示：

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

虽然没有调用 clone，但 x 依然有效。

原因在于，像整数这种编译时占用内存已知的类型存储在 stack 中，因此可以快速复制实际值。因此我们没有理由在创建变量 y 后使 x 失效。换言之，这里深复制和浅复制没有区别，调用 clone 也不会做任何与浅复制不同的事情。

Rust 有一个名为 `Copy` 的注释，可以将其放在存储在 stack 的类型上，如 integer。如果一个类型实现了 `Copy` trait，其对应变量不会 move，而是复制，使它们在复制给另一个变量后仍然有效。

如果类型或其任何部分实现了 `Drop` trait，则不允许使用 Copy 注释该类型，否则抛出编译错误。

那么，哪些类型实现了 `Copy` trait？一般来说，简单的标量值都可以实现 Copy，下面是一些实现 Copy 的类型：

- 所有整数类型，如 i32
- 布尔类型 bool
- 浮点类型，如 f64
- 字符类型 char
- 只包含实现 `Copy` 类型的 Tuple，如 `(i32, i32)` 实现了 `Copy`，但是 `(i32, String)` 没有

## Ownership 和函数

向函数传递值的机制与给变量赋值的机制类似。将变量传递给函数也有移动和复制两种情况。示例：

```rust
fn main() {
    let s = String::from("hello");  // s 进入 scope

    takes_ownership(s);  // s 的值 move 到函数，因此 s 失效

    let x = 5;           // x 进入 scope

    makes_copy(x);       // x move 到函数，但是 i32 被复制，所以后面 x 依然有效
} // 这里，x 和 s 超出 scope，但是 s 的值已经 move，所以不会发生什么

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // 这里 some_string 超出 scope，`drop` 被调用，释放内存

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

在调用 `takes_ownership` 后再使用 s，Rust 会抛出编译时错误。

## 返回值和 Scope

返回值可以转移 ownership。示例：

```rust
fn main() {
    let s1 = gives_ownership(); // gives_ownership 将其返回值移到 s1

    let s2 = String::from("hello");   // s2 进入 scope

    let s3 = takes_and_gives_back(s2); // s2 move 到 takes_and_gives_back, 该函数返回值 move 到 s3
} // s3 超出 scope 被删除，s2 已经 move，s1 超出 scope 被删除

fn gives_ownership() -> String {   // gives_ownership 将其返回值 move 到调用函数

    let some_string = String::from("yours"); // some_string 进入 scope

    some_string    // 返回 some_string，并 move 到外部调用函数
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入 scope
    a_string  // a_string is returned and moves out to the calling function
}
```

变量的 ownership 每次都遵循相同模式：将一个值赋给另一个变量会 move 它。包含 heap 数据的变量超出 scope 时，该变量的值被 drop 删除，除非数据的 ownership 已经转移到另一个变量。

每个函数都获取 ownership 然后返回 ownership 有点繁琐。如果想让一个函数使用一个值但不拥有 ownership 呢？除了函数中我们想返回的数据，我们传入的参数也需要传递回来。

Rust 通过 tuple 可以返回多个值。例如：

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

虽然这样能达到目的，但对一个常用的概念，这样太麻烦了。幸运的是，Rust 有一个特性可以在不转移所有权的情况下使用值，称为**引用**（reference）。
