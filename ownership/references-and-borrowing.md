# 引用和借用

- [引用和借用](#引用和借用)
  - [1. 简介](#1-简介)
  - [2. Mutable 引用](#2-mutable-引用)
  - [3. 悬空引用](#3-悬空引用)
  - [4. 引用规则](#4-引用规则)

Last updated: 2023-10-11, 11:19
@author Jiawei Mao
****

## 1. 简介

将参数传递给函数后，就无法继续使用。对该情况，Rust 使用引用来解决。引用类似于指针，因为它也是一个地址，根据该地址可以访问存储的数据；与指针不同的是，引用保证在引用的生命周期内指向特定类型的有效值。

**示例：** 用引用定义函数参数

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

这里使用 `&s1` 给 `calculate_length` 传递参数，在定义中使用 `&String` 声明参数类型。这里 `&` 表示引用。引用用来引用某个值但不需要拥有。图示：

@import "images/trpl04-05.png" {width="600px" title=""}

> 图 5：指向 `String s1` 的引用 `&String s`

对函数调用：

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

`&s1` 创建一个引用，该引用指向 `s1` 的值，但不拥有它。因为不拥有，所以停止使用引用，它指向的值也不会被删除。

同样，函数签名使用 `&` 来表示形参 `s` 是引用：

```rust
fn calculate_length(s: &String) -> usize { // 是字符串引用
    s.len()
} // s 超出 scope，但是因为它不拥有它指向的内容，所以不会删除其指向的内容
```

当函数的参数是引用而不是实际值时，不需要返回值来返回所有权，因为不曾拥有过所有权。

如果使用通过引用来修改变量，不起作用。例如：

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

抛出错误：

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

正如变量默认 immutable，引用也是。我们不能修改引用对象。

## 2. Mutable 引用

将函数参数修改为 `mut` 引用，就能够借用的值：

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

这里，首先把 `s` 设为 `mut`，调用 `change` 函时使用 `&mut s` 创建 mutable 引用，并更新函数签名 `some_string: &mut String` 接收一个 mutable 引用。显式指定 `change` 函数会改变它借用的值。

mutable 引用有一个很大的限制：已有 mutable 引用的值，不能有其它引用。例如，下面尝试创建 2 个 mutable 引用：

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

抛出错误：

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

这个错误告诉我们不能多次借用 mutable 变量。

该限制的好处是 Rust 可以避免在编译时防止数据竞争。满足以下三个条件，就出现数据竞争：

- 两个或多个指针同时访问相同的数据
- 至少有一个指针正在写入数据
- 没有使用同步机制控制对数据的访问

数据竞争会导致结果不确定，这类错误难以诊断和修复；Rust 在编译时拒绝带有数据竞争的代码以防止这种问题。

我们可以使用大括号来创建一个新的 scope，在其中可以额外创建 mutable 引用：

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

`r1` 和 `r2` 不在同一个 scope，所以不影响。

组合使用 mutable 和 immutable 引用也会报错：

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

错误信息：

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

当已有 immutable 引用，不能继续创建 mutable 引用。

注意，引用的 scope 从它被引入开始，到最后一次使用的地方。例如，下面的代码可以编译，因为最后一次使用 immutable 引用 println! 在引入 mutable 引用之前：

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

尽管借用错误有时令人头疼，但这是 Rust 编译器提前指出潜在的错误，避免后续麻烦。

## 3. 悬空引用

在使用指针的语言中，很容易出现释放了内存，但保留了指向该内存的指针，这类指针称为悬空指针（dangling pointer）。在 Rust 中，编译器保证引用不会变成悬空引用：编译器确保数据不会在数据引用之前超出 scope。

示例：创建悬空引用看看 Rust 的编译错误：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

编译错误：

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

该错误包含生命周期的概念，暂时忽略，只看 help 后的信息：

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

注释：

```rust
fn dangle() -> &String { // dangle 返回字符串的引用

    let s = String::from("hello"); // s 是一个 String

    &s // 返回 String s 的引用
} // s 超出 scope 被删除，危险
```

因为 `s` 是在 dangle 内部创建的，所以当 dangle 的代码完成，s 的内存被释放。但我们视图返回它的引用，导致引用指向一个无效的字符串。Rust 不会让这种事发生。

解决方法：直接放回字符串，不要返回引用

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

所有权通过返回值转移，不会释放内容。

## 4. 引用规则

引用基本规则：

- 任何时候，可以拥有一个 mutable 引用，或多个 immutable 引用
- 引用必须始终有效