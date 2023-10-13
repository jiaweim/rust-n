# 格式化输出

- [格式化输出](#格式化输出)
  - [简介](#简介)
  - [调试](#调试)
  - [显示（Display）](#显示display)


## 简介

打印操作由 `std::fmt` 里面所定义的一系列宏来处理，包括：

- format!：将格式化文本写到字符串。
- print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
- println!: 与 print! 类似，但输出结果追加一个换行符。
- eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
- eprintln!：与 eprint! 类似，但输出结果追加一个换行符。

**示例：** `{}` 被任意变量内容替换

```rust
println!("{} days", 31);
```

不加后缀，31 自动成为 i32 类型。

**示例：** 位置参数

```rust
println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
```

**示例：** 命名参数

```rust
println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over");
```

**示例：** 在 `:` 后指定特殊格式

```rust
println!("{} of {:b} people know binary, the other half don't", 1, 2);
```

```
1 of 10 people know binary, the other half don't
```

**示例：** 按指定宽度右对齐

```rust
println!("{number:>width$}", number = 1, width = 6);
```

5 个空格后连着 1.

```
     1
```

**示例：** 在数字左侧补 0

```rust
// 你可以在数字左边补 0。下面语句输出 "000001"。
println!("{number:>0width$}", number=1, width=6);
```

**示例：** println! 会检查使用的参数数量是否正确

```rust
println!("My name is {0}, {1} {0}", "Bond");
// 改正 ^ 补上漏掉的参数："James"
```

`std::fmt` 包含多种 `trait`（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：

- `fmt::Debug`：使用 {:?} 标记。格式化文本以供调试使用。
- `fmt::Display`：使用 {} 标记。以更优雅和友好的风格来格式化文本。

上例使用了 `fmt::Display`，因为标准库提供了那些类型的实现。若要打印自定义类型的文本，需要更多的步骤。

## 调试

所有的类型，若想用 `std::fmt` 的格式化打印，都要求实现至少一个可打印的 traits。仅有一些类型提供了自动实现，比如 `std` 库中的类型。其他类型都必须手动实现。

`fmt::Debug` 这个 `trait` 使这项工作变得相当简单。所有类型都能推导（`derive`，即自动创建）`fmt::Debug` 的实现。但是 `fmt::Display` 需要手动实现。

```rust
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
```

所有 `std` 库类型都天生可以使用 `{:?}` 来打印：

```rust
// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

```
12 months in a year.
"Christian" "Slater" is the "actor's" name.
Now Structure(3) will print!
Now Deep(Structure(7)) will print!
```

所以 `fmt::Debug` 确实使这些内容可以打印，但是牺牲了一些美感。Rust 也通过 `{:#?}` 提供了 “美化打印” 的功能：

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
```

```
Person {
    name: "Peter",
    age: 27,
}
```

你可以通过手动实现 fmt::Display 来控制显示效果。

## 显示（Display）

`fmt::Debug` 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。这需要通过手动实现 `fmt::Display` 来做到。`fmt::Display` 采用 `{}` 标记。实现方式看起来像这样：

```rust
// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display` 的效果可能比 fmt::Debug 简洁，但对于 std 库来说，这就有一个问题。模棱两可的类型该如何显示呢？举个例子，假设标准库对所有的 Vec<T> 都实现了同一种输出样式，那么它应该是哪种样式？下面两种中的一种吗？