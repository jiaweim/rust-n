# 生命周期

- [生命周期](#生命周期)
  - [简介](#简介)
  - [悬垂指针](#悬垂指针)
  - [借用检查](#借用检查)
  - [函数中的生命周期](#函数中的生命周期)
  - [标注生命周期](#标注生命周期)
    - [函数签名中的生命周期标注](#函数签名中的生命周期标注)
    - [生命周期标注续](#生命周期标注续)
  - [结构体中的生命周期](#结构体中的生命周期)
  - [生命周期消除](#生命周期消除)
    - [三条消除规则](#三条消除规则)
  - [方法中的生命周期](#方法中的生命周期)
  - [静态生命周期](#静态生命周期)

@author Jiawei Mao
***

## 简介

生命周期，就是引用的作用域。大多时候，我们无需手动声明生命周期，编译器会自动推导。以类型来类比：

- 和编译器大部分时候可以自动推导类型一样，编译器大多数时候也可以自动推导生命周期
- 在多种类型存在时，编译器往往要求手动标明类型，当多个生命周期存在，且编译器无法推导出某个引用的生命周期时，也需要手动标明生命周期

## 悬垂指针

生命周期的主要作用是避免**悬垂引用**，它会导致程序引用了本不该引用的数据：

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

这段代码有几点值得注意:

- `let r;` 的声明方式貌似存在使用 null 的风险，实际上，当我们不初始化它就使用时，编译器会报错
- `r` 引用内部花括号中的 `x` 变量，但是 `x` 会在内部花括号 `}` 处被释放，因此回到外部花括号后， `r` 会引用一个无效的 `x`

此处 `r` 就是一个悬垂指针，它引用了提前被释放的变量 `x` ，可以预料到，这段代码会报错：

```sh
error[E0597]: `x` does not live long enough // `x` 活得不够久
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough // 被借用的 `x` 活得不够久
8  |         }
   |         - `x` dropped here while still borrowed // `x` 在这里被丢弃，但是它依然还在被借用
9  |
10 |         println!("r: {}", r);
   |                           - borrow later used here // 对 `x` 的借用在此处被使用
```

这里 `r` 拥有更大的作用域，或者说**活得更久**。如果 Rust 不阻止该悬垂引用，那么当 x 被释放后，`r` 所引用的值就不再合法，导致程序异常，且该异常很难被发现。

## 借用检查

为了保证所有权和借用的正确性，Rust 使用借用检查器(Borrow checker)检查程序的借用正确性：

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

这段代码和之前一模一样，只是增加变量生命周期的注释。这里， `r` 变量的生命周期为 'a ， x 的生命周期为 'b ，从图示上可以看出生命周期 'b 比 'a 小很多。

在编译期，Rust 比较两个变量的生命周期，发现 `r` 明明拥有生命周期 'a ，但是却引用了一个小得多的生命周期 'b ，因此编译器认为程序存在风险，拒绝运行。

如果想通过编译，只要 'b 比 'a 大就好。`x` 变量只要比 `r` 活得久，`r` 就能随意引用 x 且不会存在危险：

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

## 函数中的生命周期

**示例：** 返回两个字符串切片中较长的那个

该函数的参数是两个字符串切片，返回值也是字符串切片：

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这段 `longest` 会报错：

```sh
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter // 参数需要一个生命周期
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is
  borrowed from `x` or `y`
  = 帮助： 该函数的返回值是一个引用类型，但是函数签名无法说明，该引用是借用自 `x` 还是 `y`
help: consider introducing a named lifetime parameter // 考虑引入一个生命周期
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

这里的主要问题是，编译器不知道函数返回 x 还是 y，无法分析函数调用后的引用生命周期。

关键是，我们也不知道返回 x 还是 y，咋办？

在存在多个引用时，编译器有时无法自动推导生命周期，此时需要我们手动标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析。

## 标注生命周期

!!! note
    标注生命周期不会改变引用的作用域。

标记的生命周期只是为了取悦编译器，让编译器不要难为我们。

例如一个变量，只能活一个花括号，那么就算你给它标注一个活全局的生命周期，它还是会在前面的花括号结束处被释放掉，并不会真的全局存活。

生命周期标注以 `'` 开头，名称一般是一个小写字母，大多数人用 `'a` 作为生命周期名称。对引用类型参数，生命周期位于引用符号 `&` 之后，并用一个空格来将生命周期和引用参数分隔:

```rust
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用
```

生命周期标注告诉编译器多个引用之间的关系。

例如，有一个函数，它的第一个参数 `first` 是一个 `i32` 类型引用，生命周期为 `'a`，另一个参数 `second` 也是 `i32` 类型引用，生命周期也是 `'a`。此处生命周期标注仅仅说明，这两个参数 `first` 和 `second` 至少活得和 'a 一样久，至于到底活多久或者哪个活得更久，抱歉我们都无法得知：

```rust
fn useless<'a>(first: &'a i32, second: &'a i32) {}
```

### 函数签名中的生命周期标注

继续之前的 `longest` 函数，从两个字符串切片中返回较长的那个：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

要点：

- 和泛型一样，使用生命周期参数，需要先声明 `<'a>`
- `x`、`y` 和返回值至少活得和 `'a` 一样久 (因为返回值要么是 x ，要么是 y )

该函数签名表明两个参数至少和生命周期 `'a` 活得一样久，同时函数的返回引用也不小于 `'a`。实际上，

这意味着返回值的生命周期与参数生命周期中的较小值一致：虽然两个参数的生命周期都是标注了 `'a`，但是这两个参数的真实生命周期可能不一样(生命周期 `'a` 不代表生命周期等于 'a ，而是大于等于 'a )。

!!! note
    通过函数签名指定参数的生命周期时，并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。

因此 `longest` 函数并不知道 `x` 和 `y` 能活多久，但是知道它们的作用域至少不小于 `'a`。

当把具体的引用传给 `longest`，生命周期 `'a` 的大小就是 `x` 和 `y` 作用域的重合部分，换句话说， `'a` 的大小等于 x 和 y 中较小的那个。由于返回值的生命周期也被标记为 `'a` ，因此返回值的生命周期也是 x 和 y 中作用域较小的那个。

**示例：**

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

这里 `string1` 的作用域到 `main` 函数的结束，`string2` 的作用域到内部花括号的结束 `}` ，那么根据之前的理论， 'a 是两者中作用域较小的那个，也就是 'a 的生命周期等于 `string2` 的生命周期，同理，由于函数返回的生命周期也是 `'a` ，可以得出函数返回的生命周期也等于 `string2` 的生命周期。

**示例：** 验证返回值的生命周期

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

报错：

```sh
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

在上述代码中，`result` 必须要活到 `println!` 处，因为 `result` 的生命周期是 `'a` ，因此 `'a` 必须持续到 `println!`。

在 `longest` 函数中， `string2` 的生命周期也是 `'a` ，说明 `string2` 也必须活到 `println!`处，可是 `string2` 在代码中实际上只能活到内部语句块的花括号处 `}`，小于它应该具备的生命周期 `'a` ，因此编译出错。

### 生命周期标注续

使用生命周期的方式往往取决于函数的功能，如之前的 `longest` 函数，如果它永远只返回第一个参数 `x`，生命周期的标注该如何修改?

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

这里 `y` 完全没有被使用，因此 `y` 的生命周期与 `x` 和返回值的生命周期没有任何关系，因此不必再为 `y` 标注生命周期。

函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：

- 函数参数的生命周期
- 函数体中某个新建引用的生命周期

若是后者，就是典型的悬垂引用：

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

上面函数返回值和参数 `x`，`y` 没有任何关系，而是引用了函数体内创建的字符串，很显然函数会报错：

```sh
error[E0515]: cannot return value referencing local variable `result` // 返回值result引用了本地的变量
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here
```

主要问题就在于，`result` 在函数结束后就被释放，但是在函数结束后对 `result` 的引用依然在使用。因此无法指定合适的生命周期来让编译通过，从而避免了悬垂引用。

对以上情况，最好的办法就是返回内部字符串的所有权，然后把字符串的所有权转移给调用者：

```rust
fn longest<'a>(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

fn main() {
   let s = longest("not", "important");
}
```

!!! summary
    生命周期标注将函数的引用参数和返回值的作用域关联在一起，辅助编译器确定变量的作用域。

## 结构体中的生命周期

为引用标注生命周期，结构体中也能使用引用：

```rust{.line-numbers}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

`ImportantExcerpt` 结构体中有一个引用类型的字段 `part`，因此需要为它标注上生命周期。结构体的生命周期标注语法跟泛型参数语法很像，需要对生命周期参数进行声明 `<'a>` 。该生命周期标注说明，结构体 `ImportantExcerpt` 所引用的字符串 `str` 必须比该结构体活得更久。

在 `main` 中，`ImportantExcerpt` 的生命周期从第 8 行开始，到 `main` 函数末尾结束，而该结构体引用的字符串从第 6 行开始，也是到 `main` 函数末尾结束，所以结构体引用的字符串活得比结构体久，符合编译器对生命周期的要求，因此编译通过。

与之相反，下面的代码就无法通过编译：

```rust
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}",i);
}
```

可以看到结构体比它引用的字符串活得更久，引用字符串在内部语句块末尾 `}` 被释放后，`println!` 依然在外面使用了该结构体，因此会导致无效的引用，编译报错：

```sh
error[E0597]: `novel` does not live long enough
  --> src/main.rs:10:30
   |
10 |         let first_sentence = novel.split('.').next().expect("Could not find a '.'");
   |                              ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
...
14 |     }
   |     - `novel` dropped here while still borrowed
15 |     println!("{:?}",i);
   |                     - borrow later used here
```

## 生命周期消除

对编译器而言，每一个引用类型都有一个生命周期，但是大多时候无需标注生命周期。例如：

```rust{.line-numbers}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

该函数的参数和返回值都是引用类型，尽管没有显式的为其标注生命周期，编译依然可以通过。因为编译器为了简化用户的使用，运用了生命周期消除。

对于 `first_word` 函数，它的返回值是一个引用类型，那么该引用只有两种情况：

- 从参数获取
- 从函数体内部新创建的变量获取

如果是后者，就会出现悬垂引用，最终被编译器拒绝，如果返回值的引用是函数参数，这意味着参数和返回值的生命周期是一样的。此时就算不标注生命周期，也不会产生歧义。

要点：

- 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，此时需要手动标注生命周期
- 函数或者方法中，参数的生命周期被称为 `输入生命周期` ，返回值的生命周期被称为 `输出生命周期`

### 三条消除规则

编译器根据三条规则来确定哪些场景不需要显式标注生命周期。

其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示需要手动标注生命周期。

1. **每个引用参数拥有独自的生命周期**

例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。

2. 若只有一个输入生命周期（函数只有一个引用类型参数），那么该生命周期会被赋给所有输出生命周期，即所有返回值的生命周期都等于该输入的生命周期

例如函数 `fn foo(x: &i32) -> &i32`， `x` 参数的生命周期会被自动赋给返回值 `&i32`，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`。

3. 若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期

但是，若一个方法，它的返回值的生命周期跟参数 `&self` 不一样怎么办？答案很简单：手动标注生命周期，因为这些规则只是用于没有标注生命周期的默认选择，当标注生命周期后，以标注为准。

**示例：**

```rust
fn first_word(s: &str) -> &str { // 实际项目中的手写代码
```

编译器会先应用规则一，为每个参数标注一个生命周期：

```rust
fn first_word<'a>(s: &'a str) -> &str { // 编译器自动为参数添加生命周期
```

然后，应用规则二，因为只有一个输入生命周期，因此输出生命周期与该输入生命周期相同：

```rust
fn first_word<'a>(s: &'a str) -> &'a str { // 编译器自动为返回值添加生命周期
```

到这里，编译器成功为函数签名中的所有引用添加了生命周期，编译通过。

**示例：**

```rust
fn longest(x: &str, y: &str) -> &str { // 实际项目中的手写代码
```

编译器首先按规则一标注参数的生命周期：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

此时，无法继续应用规则二，也不符合规则三。编译器无法为返回值标注合适的生命周期，因此会报错，提示需手动标注生命周期：

```sh
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:47
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
  |                       -------     -------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
note: these named lifetimes are available to use
 --> src/main.rs:1:12
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
  |            ^^  ^^
help: consider using one of the available lifetimes here
  |
1 | fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'lifetime str {
  |                                                +++++++++
```

## 方法中的生命周期

以泛型为例：

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

为具有生命周期的结构体实现方法的语法跟泛型参数语法很相似：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

要点：

- `impl` 中必须使用结构体的完整名称，包括 `<'a>`，因为生命周期标注也是结构体类型的一部分！
- 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则

下面的例子展示了第三规则应用的场景：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

首先，编译器应用第一规则，给予每个输入参数一个生命周期:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

需要注意的是，编译器不知道 `announcement` 的生命周期到底多长，因此它无法简单的给予它生命周期 `'a` ，而是重新声明了一个全新的生命周期 `'b`。

接着，编译器应用第三规则，将 `&self` 的生命周期赋给返回值 `&str`：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

在结束这块儿内容之前，再来做一个有趣的修改，将方法返回的生命周期改为 `'b` ：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

此时，编译器会报错，因为编译器无法知道 `'a` 和 `'b` 的关系。 `&self` 生命周期是 `'a`，那么 `self.part` 的生命周期也是 `'a`，但是好巧不巧的是，我们手动为返回值 `self.part` 标注了生命周期 `'b`，因此编译器需要知道 `'a` 和 `'b` 的关系。

由于 `&'a self` 是被引用的一方，因此引用它的 `&'b str` 必须要活得比它短，否则会出现悬垂引用。因此说明生命周期 `'b` 必须要比 `'a` 小，只要满足了这一点，编译器就不会再报错：

```rust
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

说明：

- `'a: 'b` 是生命周期约束语法，跟泛型约束类似，用于说明 `'a` 必须比 `'b` 活得久
- 可以把 `'a` 和 `'b` 在同一个地方声明（如上），或者分开声明但通过 `where 'a: 'b` 约束生命周期关系，如下：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

总之，实现方法比想象中简单：加一个约束，就能暗示编译器，尽管引用吧，反正我想引用的内容比我活得久。

## 静态生命周期

在 Rust 中有一个非常特殊的生命周期 `'static`，拥有该生命周期的引用可以和整个程序活得一样久。

在之前我们学过字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中，因此这些字符串变量
全部具有 'static 的生命周期：

