# 字符、布尔和单元类型

- [字符、布尔和单元类型](#字符布尔和单元类型)
  - [字符类型](#字符类型)
  - [Boolean 类型](#boolean-类型)
  - [单元类型](#单元类型)

Last updated: 2023-10-17, 11:27
@author Jiawei Mao
****

## 字符类型

`char` 表示字符类型。示例：

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

Unicode 标量值范围为：[U+0000, U+D7FF] 和 [U+E000, U+10FFFF]。

## Boolean 类型

Boolean 类型有 2 个值：true 和 false。占用 1 个字节大小，使用 `bool` 注释类型。

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## 单元类型

**单元类型**就是 `()`，唯一的值也是 `()`。例如 `main` 函数就返回单元类型；常用的 `println!()` 也返回 `()`。

再比如，可以用 `()` 作为 `map` 的值，表示不关注具体的值，即用来占位，但完全不占用内存。

```rust
// 让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
```


在 Rust 中没有返回值的函数称为**发散函数**（diverge function），即无法收敛的函数。

