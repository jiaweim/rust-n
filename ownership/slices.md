# Slice

- [Slice](#slice)
  - [1. 简介](#1-简介)
  - [2. String 切片](#2-string-切片)
    - [2.1. 字符串字面量为切片](#21-字符串字面量为切片)
    - [2.2. 字符串切片作为参数](#22-字符串切片作为参数)
  - [1.3. 其它切片](#13-其它切片)

Last updated: 2023-10-11, 15:38
@author Jiawei Mao
****

## 1. 简介

切片（slice）用来引用集合中的连续元素序列，而不是整个集合。**切片是一种引用**，因此它没有所有权。

先给出一个编程问题：编写一个函数，接收一个由空格分隔的单词组成的字符串，返回在该字符串找到的第一个字符串；如果在字符串中没有找到空格，那么整个字符串是一个单词，返回整个字符串。

先看看，如果不使用切片，如何编写这个函数：

```rust
fn first_word(s: &String) -> ?
```

`first_word` 的参数为 `&String`，因为不需要所有权。那么应该返回什么？返回单词结尾的下标：

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

因为需要遍历字符串的字符，所以使用 `as_bytes` 将 String 转换为 bytes 数组。

```rust
let bytes = s.as_bytes();
```

然后用 iter 创建字节数组的迭代器：

```rust
for (i, &item) in bytes.iter().enumerate() {
```

## 2. String 切片

字符串切片是对字符串一部分的引用，类似：

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

这里 `hello` 是对字符串 `s` `[0..5]` 这一部分的引用。

切片范围通过 `[start_index..end_index]` 指定。在内部，slice 存储切片的起始位置和长度。所以 `let world = &s[6..11];`，world 是一个长度为 5 的切片，指向 s 的索引 6。如下图所示：

@import "images/trpl04-06.png" {width="360px" title=""}

用 Rust 的 `..` 语法，如果索引从 0 开始，可以省掉 0：

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

类似地，如果取到字符串末尾，也可以省略：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

如果切片整个字符串，起始和末尾索引都不需要：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

下面根据字符串切片重写 first_word 函数：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

因为切片是对字符串的引用，编译器不允许在引用 scope 内清理字符串，因此在切片 scope 范围内清理字符串会报错：

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

编译错误：

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

根据借用规则，如果我们有一个 immutable 引用，就不能同时创建一个 mutable 引用。clear 需要截断字符串，所以它需要 mutable 引用。`println!` 在 clear 之后使用 word 引用，因此 immutable 引用在此必须依然有效。

### 2.1. 字符串字面量为切片

字符串字面量存储在二进制文件中。例如：

```rust
let s = "Hello, world!";
```

这里 s 的 类型为 `&str`：即 s 是指向二进制文件特定点的切片。这也是为什么字符串字面量是 immutable 的，`&str` 是 immutable 引用。

### 2.2. 字符串切片作为参数

了解字面量和字符串切片后，我们继续对 first_word 进行改进，其签名如下：

```rust
fn first_word(s: &String) -> &str {
```

更好的方式是使用 &str 作为参数：

```rust
fn first_word(s: &str) -> &str {
```

对字符串切片，可以直接传入函数；对字符串，可以传递该字符串的切片或引用。该灵活性利用了 deref 强制转换。

定义一个接受字符串切片而不是字符串引用的函数，可以让 API 更通用，同时不会失去任何功能：

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

## 1.3. 其它切片

切片也可用于数组：

```rust
let a = [1, 2, 3, 4, 5];
```

切片操作：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

该切片类型为 `&[i32]`。其工作方式与字符串一样。