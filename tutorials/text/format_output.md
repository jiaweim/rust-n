# 格式化输出

- [格式化输出](#格式化输出)
  - [println!](#println)
  - [print!, println!, format!](#print-println-format)
  - [eprint!，eprintln!](#eprinteprintln)
  - [{}](#)
    - [Debug 特征](#debug-特征)
    - [Display 特征](#display-特征)
    - [为自定义类型实现 Display 特征](#为自定义类型实现-display-特征)
    - [为外部类型实现 Display 特征](#为外部类型实现-display-特征)
  - [位置参数](#位置参数)
  - [具名参数](#具名参数)
  - [格式化参数](#格式化参数)
    - [宽度](#宽度)
      - [字符串填充](#字符串填充)
      - [数字填充：符号和0](#数字填充符号和0)
    - [对齐](#对齐)
    - [精度](#精度)
    - [进制](#进制)
    - [指数](#指数)
    - [指针地址](#指针地址)
    - [转义](#转义)
  - [在格式化字符串时捕获环境中的值（Rust 1.58+）](#在格式化字符串时捕获环境中的值rust-158)
  - [简介](#简介)
  - [调试](#调试)
  - [显示（Display）](#显示display)

2023-10-30, 20:02
@author Jiawei Mao
****

## println!

```rust
println!("Hello");                 // => "Hello"
println!("Hello, {}!", "world");   // => "Hello, world!"
println!("The number is {}", 1);   // => "The number is 1"
println!("{:?}", (3, 4));          // => "(3, 4)"
println!("{value}", value=4);      // => "4"
println!("{} {}", 1, 2);           // => "1 2"
println!("{:04}", 42);             // => "0042" with leading zeros
```

可以看到，`println!` 宏接受可变参数，**第一个参数**是字符串常量，为格式化字符串，其中包含占位符 `{}`，会被 `println!` 后面的参数依次替换。

## print!, println!, format!

它们是 Rust 格式化输出的主力：

- `print!` 将格式化文本输出到标准输出，不带换行符
- `println!` 同上，但在行末尾带换行符
- `format!` 将格式化文本输出到 `String` 字符串

其中 `println!` 和 `format!` 最常用，前者常用于调试输出，后者常用于生成格式化的字符串：

```rust
fn main() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
}
```

说明：`s1` 是 `format!` 生成的 `String` 字符串，最终输出为

```sh
hello, world
hello, world!
```

## eprint!，eprintln!

除了三大金刚外，还有两大护法，使用方式跟 print!，println! 很像，但是它们输出到标准错误输出：

```rust
eprintln!("Error: Could not complete task")
```

它们仅应该被用于输出错误信息和进度信息，其它场景都应该使用 print! 系列。

## {}

与其它语言常用的 %d，%s 不同，Rust 特立独行地选择了 {} 作为格式化占位符（说到这个，有点想吐槽下，Rust 中自创的概念其实还挺多的，真不知道该夸奖还是该吐槽-,-），事实证明，这种选择非常正确，它帮助用户减少了很多使用成本，你无需再为特定的类型选择特定的占位符，统一用 {} 来替代即可，剩下的类型推导等细节只要交给 Rust 去做。

与 {} 类似，{:?} 也是占位符：

- {} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
- {:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景

其实两者的选择很简单，当你在写代码需要调试时，使用 {:?}，剩下的场景，选择 {}。

### Debug 特征

为了方便调试，大多数 Rust 类型都实现了 `Debug` 特型或者支持派生该特型，从而可以使用 `{:?}` 格式化类型。

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "sunface".to_string(), age: 18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}
```

对于数值、字符串、数组，可以直接使用 `{:?}` 进行输出，但是对于结构体，需要派生Debug特征后，才能进行输出，总之很简单。

### Display 特征

与大部分类型实现了 `Debug` 不同，实现 `Display` 特征的类型没有那么多，往往需要自定义想要的格式化方式：

```rust
let i = 3.1415926;
let s = String::from("hello");
let v = vec![1, 2, 3];
let p = Person {
    name: "sunface".to_string(),
    age: 18,
};
println!("{}, {}, {}, {}", i, s, v, p);
```

运行后可以看到 v 和 p 都无法通过编译，因为没有实现 Display 特征，但是你又不能像派生 Debug 一般派生 Display，只能另寻他法：

- 使用 {:?} 或 {:#?}
- 为自定义类型实现 Display 特征
- 使用 newtype 为外部类型实现 Display 特征

下面来一一看看这三种方式。

{:#?} 与 {:?} 几乎一样，唯一的区别在于它能更优美地输出内容：

```sh
// {:?}
[1, 2, 3], Person { name: "sunface", age: 18 }

// {:#?}
[
    1,
    2,
    3,
], Person {
    name: "sunface",
}
```

因此对于 Display 不支持的类型，可以考虑使用 {:#?} 进行格式化，虽然理论上它更适合进行调试输出。

### 为自定义类型实现 Display 特征

如果你的类型是定义在当前作用域中的，那么可以为其实现 Display 特征，即可用于格式化输出：

```rust
struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
fn main() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}
```

如上所示，只要实现 Display 特征中的 fmt 方法，即可为自定义结构体 Person 添加自定义输出：

```sh
大佬在上，请受我一拜，小弟姓名sunface，年芳18，家里无田又无车，生活苦哈哈
```

### 为外部类型实现 Display 特征

在 Rust 中，无法直接为外部类型实现外部特征，但是可以使用newtype解决此问题：

```rust
struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}
fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}
```

`Array` 就是我们的 `newtype`，它将想要格式化输出的 Vec 包裹在内，最后只要为 Array 实现 Display 特征，即可进行格式化输出：

```sh
数组是：[1, 2, 3]
```

至此，关于 {} 与 {:?} 的内容已介绍完毕，下面让我们正式开始格式化输出的旅程。

## 位置参数

除了按照依次顺序使用值去替换占位符之外，还能让指定位置的参数去替换某个占位符，例如 {1} ，
表示用第二个参数替换该占位符(索引从 0 开始)：

```rust
fn main() {
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112
}
```

## 具名参数

除了像上面那样指定位置外，我们还可以为参数指定名称：

```rust
fn main() {
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
}
```

需要注意的是：带名称的参数必须放在不带名称参数的后面，例如下面代码将报错：

```rust
println!("{abc} {1}", abc = "def", 2);
```

```sh
error: positional arguments cannot follow named arguments
 --> src/main.rs:4:36
   |
 4 | println!("{abc} {1}", abc = "def", 2);
   |                             -----  ^ positional arguments must be before named arguments
   |                             |
   |                             named argument
```

## 格式化参数

格式化输出，意味着对输出格式会有更多的要求，例如只输出浮点数的小数点后两位：

```rust
fn main() {
    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);
}
```

上面代码只输出小数点后两位。同时我们还展示了 {} 和 {:?} 的用法，后面如无特殊区别，就只针对 {} 提供格式化参数说明。

接下来，让我们一起来看看 Rust 中有哪些格式化参数。

### 宽度

宽度用来指示输出目标的长度，如果长度不够，则进行填充和对齐：

#### 字符串填充

字符串格式化默认使用空格进行填充，并且进行左对齐。

```rust
fn main() {
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
}
```

#### 数字填充：符号和0

数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐。

```rust
fn main() {
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
}
```

### 对齐

```rust
fn main() {
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");
}
```

### 精度

精度可以用于控制浮点数的精度或者字符串的长度

```rust
fn main() {
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
}
```

### 进制

可以使用 `#` 号来控制数字的进制输出：

- `#b`, 二进制
- `#o`, 八进制
- `#x`, 小写十六进制
- `#X`, 大写十六进制
- `x`, 不带前缀的小写十六进制

```rust
fn main() {
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
}
```

### 指数

```rust
fn main() {
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
}
```

### 指针地址

```rust
let v= vec![1, 2, 3];
println!("{:p}", v.as_ptr()) // => 0x600002324050
```

### 转义

有时需要输出 {和}，但这两个字符是特殊字符，需要进行转义：

```rust
fn main() {
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}" 
    println!(" Hello \"{{World}}\" ");

    // 下面代码会报错，因为占位符{}只有一个右括号}，左括号被转义成字符串的内容
    // println!(" {{ Hello } ");
    // 也不可使用 '\' 来转义 "{}"
    // println!(" \{ Hello \} ")
}
```

## 在格式化字符串时捕获环境中的值（Rust 1.58+）

在以前，想要输出一个函数的返回值，你需要这么做：

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let p = get_person();
    println!("Hello, {}!", p);                // implicit position
    println!("Hello, {0}!", p);               // explicit index
    println!("Hello, {person}!", person = p);
}
```

问题倒也不大，但是一旦格式化字符串长了后，就会非常冗余，而在 1.58 后，我们可以这么写：

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let person = get_person();
    println!("Hello, {person}!");
}
```

是不是清晰、简洁了很多？甚至还可以将环境中的值用于格式化参数:

```rust
let (width, precision) = get_format();
for (name, score) in get_scores() {
  println!("{name}: {score:width$.precision$}");
}
```

但也有局限，它只能捕获普通的变量，对于更复杂的类型（例如表达式），可以先将它赋值给一个变量或使用以前的 name = expression 形式的格式化参数。 目前除了 panic! 外，其它接收格式化参数的宏，都可以使用新的特性。对于 panic! 而言，如果还在使用 2015版本 或 2018版本，那 panic!("{ident}") 依然会被当成 正常的字符串来处理，同时编译器会给予 warn 提示。而对于 2021版本 ，则可以正常使用:

```rust
fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let person = get_person();
    panic!("Hello, {person}!");
}
```

```sh
thread 'main' panicked at 'Hello, sunface!', src/main.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

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