# 字符、布尔和单元类型

- [字符、布尔和单元类型](#字符布尔和单元类型)
  - [1. 字符类型](#1-字符类型)
  - [2. Boolean 类型](#2-boolean-类型)
  - [3. 单元类型](#3-单元类型)

Last updated: 2023-10-17, 11:27
@author Jiawei Mao
****

## 1. 字符类型

`char` 表示字符类型，占 4 字节 32 位。示例：

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

`char` 字面量使用单引号，而字符串字面量使用双引号。由于 Unicode 是 4 字节编码，所以 Rust 字符类型 `char` 占用 4 个字节。

```rust
fn main() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}
```

```sh
$ cargo run
   Compiling ...
字符'中'占用了4字节的内存大小
```

Unicode 标量值范围为：[U+0000, U+D7FF] 和 [U+E000, U+10FFFF]

- 如果字符编码在 U+0000 到 U+007F 之间，即 ASCII 字符集内，就可以把字符写为 '\xHH'，其中 HH 是两个十六进制数。例如，字符字面量 '*' 和 '\x2A' 是等效的，因为字符 `*` 的编码是 42 或十六进制 2A。
- 任何 Unicode 字符可以写成 '\u{HHHHHH}'，其中 HHHHHH 最多 6 个十六进制数，可以用下划线分组。

!!! info
    Rust 对单独的字符使用 char 类型，但对字符串使用 UTF-8 编码。因此，String 将其文本表示为 UTF-8 字节序列，而不是字符数组。

## 2. Boolean 类型

Boolean 类型有 2 个值：`true` 和 `false`。占用 1 个字节大小，使用 `bool` 注释类型。

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

将 bool 值转换为整数：

```rust
assert_eq!(false as i32, 0);
assert_eq!(true as i32, 1);
```

## 3. 单元类型

**单元类型**就是 `()`，唯一的值也是 `()`。例如 `main` 函数就返回单元类型；常用的 `println!()` 也返回 `()`。

也可以用 `()` 作为 `map` 的值，表示不关注具体的值，即用来占位，但完全不占用内存。

```rust
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
```


在 Rust 中没有返回值的函数称为**发散函数**（diverge function），即无法收敛的函数。

