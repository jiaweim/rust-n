# use

- [use](#use)
  - [1. 简介](#1-简介)
  - [2. 引入方式](#2-引入方式)
    - [2.1. 绝对路径引入模块](#21-绝对路径引入模块)
    - [2.2. 相对路径引入模块中的函数](#22-相对路径引入模块中的函数)
    - [2.3. 引入模块还是函数](#23-引入模块还是函数)
  - [3. 避免同名引用](#3-避免同名引用)
    - [3.1. 模块::函数](#31-模块函数)
    - [3.2. as 别名](#32-as-别名)
  - [4. 引入项再导出](#4-引入项再导出)
  - [5. 使用第三方包](#5-使用第三方包)
    - [5.1. crates.io, lib.rs](#51-cratesio-librs)
  - [6. 使用 {} 简化引入方式](#6-使用--简化引入方式)
    - [6.1. self](#61-self)
  - [7. 使用 \* 引入模块的所有项](#7-使用--引入模块的所有项)
  - [8. 受限的可见性](#8-受限的可见性)
    - [8.1. 限制可见性语法](#81-限制可见性语法)
    - [8.2. 示例](#82-示例)

2023-10-30, 16:52
@author Jiawei Mao
****

## 1. 简介

类似 `crate::front_of_house::hosting::add_to_waitlist` 的函数调用形式过于繁琐，使用 `use` 关键字把路径引入当前作用域，调用时可以省略前缀，简化代码。

## 2. 引入方式

在 Rust 中，引入模块内容有两种方式：绝对路径和相对路径。

### 2.1. 绝对路径引入模块

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

这里使用 `use` 将 `hosting` 模块引入当前作用域，然后只需使用 `hosting::add_to_waitlist` 即可调用目标模块中的函数，相比 `crate::front_of_house::hosting::add_to_waitlist()` 要简单的多。

还能更简单吗？

### 2.2. 相对路径引入模块中的函数

使用相对路径引入模块中的 `add_to_waitlist` 函数：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

此时调用 `add_to_waitlist()` 函数无需前缀。

### 2.3. 引入模块还是函数

从调用简洁性来讲，引入函数更甚一筹，但是引入模块有其优点：

- 需要引入同一个模块的多个函数
- 作用域中存在同名函数

对以上两种情况，使用 `use front_of_house::hosting;` 引入模块要比 `use front_of_house::hosting::add_to_waitlist;` 引入函数更好。

**示例：** 如果想使用 `HashMap`，那么直接引入该结构体比引入模块更好

因为在 `collections` 模块中，我们只需要使用 `HashMap` 结构体：

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

严格来说，对引用方式没有必须要遵守的惯例，主要取决于个人喜好。

## 3. 避免同名引用

### 3.1. 模块::函数

- 引入模块，避免同名

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

上面使用模块引入的方式，即 `Result` 通过 `模块::Result` 的方式进行调用，来避免同名问题。

可以看出，避免同名冲突的关键是使用**父模块的方式来调用**，除此之外，还可以给引入的项起一个别名。

### 3.2. as 别名

对同名冲突问题，还可以使用 `as` 关键字来解决，它可以赋予引入项一个新的名称：

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

如上所示，首先通过 `use std::io::Result` 将 `Result` 引入到作用域，然后使用 `as` 给它一个新名称 `IoResult`，这样就没有命名冲突了：

- `Result` 代表 `std::fmt::Result`
- `IoResult` 代表 `std:io::Result`

## 4. 引入项再导出

当外部模块的项 `A` 被引入当前模块，其可见性自动被设为私有，如果希望其它外部代码引用当前模块的项 `A`，可以对它进行再导出：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

如上，使用 `pub use` 即可实现。这里 `use` 表示引入 `hosting` 模块到当前作用域，`pub` 表示将该引入的内容设为可见。

当希望将内部的实现细节隐藏起来或者按照某个目的组织代码时，可以使用 `pub use` 再导出，例如统一使用一个模块来提供对外的 API，那该模块就可以引入其它模块中的 API，然后进行再导出，最终对于用户来说，所有的 API 都是由一个模块统一提供的。

## 5. 使用第三方包

引入外部依赖项：

1. 修改 `Cargo.toml` 文件，在 `[dependencies]` 区域添加一行：`rand = "0.8.3"`
2. 此时如果使用了 VSCode 的 rust-analyzer 插件或 RustRover，则会自动拉取该库，等它完成后进行下一步

此时，`rand` 包已经添加到依赖项，下一步在代码中使用：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

这里使用 `use` 引入了第三方包 `rand` 中的 `Rng` 特征，因为需要调用的 `gen_range` 方法定义在该特征中。

### 5.1. crates.io, lib.rs

Rust 社区贡献了大量高质量的第三方包，可以在 crates.io 或者 lib.rs 中检索和使用。

目前来说，查找包更推荐 lib.rs，它搜索功能更强大，内容展示更合理，但是下载依赖包还是得用 crates.io。

你可以在网站上搜索 `rand` 包，查看它的文档，看看其使用方式是否和之前引入方式一致。

在网上找到想要的包，然后将包名和版本信息写入 `Cargo.toml` 文件。

## 6. 使用 {} 简化引入方式

- 对于这种一行一行的引入方式

```rust
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
```

可以使用 `{}` 一起引入，从而减少 `use` 的使用：

```rust
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io};
```

- 对同时引入模块和模块中的项

```rust
use std::io;
use std::io::Write;
```

也可以使用 `{}` 进行简化:

```rust
use std::io::{self, Write};
```

这里 `self` 表示 `std::io` 模块本身。

### 6.1. self

上面使用到了模块章节提到的 self 关键字，用来替代模块自身，结合上一节中的 self，可以得出它在模块中的两个用途：

- use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略
- use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy

## 7. 使用 * 引入模块的所有项

对之前一行一行引入 `std::collections` 的方式，还可以使用：

```rust
use std::collections::*;
```

该方式引入 `std::collections` 模块下的所有 `pub` 内容，自然包含了 `HashMap`，`HashSet` 等集合类型。

不过要谨慎使用 `*`，因为你很难知道到底引入了哪些内容，又有哪些会和程序中的名称冲突：

```rust
use std::collections::*;

struct HashMap;
fn main() {
   let mut v =  HashMap::new();
   v.insert("a", 1);
}
```

这里 `std::collection::HashMap` 被 `*` 引入当前作用域，但是由于存在同名结构体，该结构体没有 `HashMap::new` 函数，而对于编译器来说，本地同名类型的优先级更高。

在实际项目中，`*` 引用方式往往用于写测试代码，把所有东西一次性引入到 `tests` 模块中。

## 8. 受限的可见性

可见性控制模块中哪些内容可以被外部看见，但是在实际使用时，光被外面看到还不行，还想控制哪些人能看，这就是 Rust 提供的受限可见性。

例如，包是一个模块树，通过 `pub(crate) item;` 实现：`item` 只在当前包内可见，外部包无法引用 `item`。

如果想要让某一项可以在整个包中使用，有两种办法：

- 在 crate-root 中定义一个非 `pub` 类型的 `X`：父模块的项对子模块是可见的，因此 crate-root 中的项对模块树上的所有模块都可见
- 在子模块中定义一个 `pub` 类型的 `Y`，同时通过 `use` 将其引入到 crate-root

```rust
mod a {
    pub mod b {
        pub fn c() {
            println!("{:?}",crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}",Y);
}
```

但是有时我们会遇到这两种方法都好用的情况。例如希望对于某些模块可见，对其它模块不可见：

```rust
// 目标：`a` 导出 `I`、`bar` and `foo`，其他的不导出
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        mod c {
            const J: i32 = 4;
        }
    }
}
```

这段代码会报错，因为子模块中的项对父模块不可见。在 `semisecret` 方法中，`a -> b -> c` 形成了父子模块链，因此 `c` 中的 `J` 对 `a` 模块不可见，所以 `use self::b::c::J;` 报错。

使用之前的可见性方式，实现保持 `J` 私有，同时让 `a` 继续使用 `semisecret` 函数，只能将该函数移到 `c` 模块，然后用 `pub use` 将 `semisecret` 函数再导出：

```rust
pub mod a {
    pub const I: i32 = 3;

    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4;
            pub fn semisecret(x: i32) -> i32 {
                x + J
            }
        }
    }
}
```

这段代码问题不大，但是有些破坏之前的逻辑，如果想保持代码逻辑，同时又只让 `J` 在 `a` 内可见，该怎么办？

```rust
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}
```

通过 `pub(in crate::a)` 指定了模块 `c` 和常量 `J` 的只是 a 模块可见，`a` 之外的模块访问不到它们。

### 8.1. 限制可见性语法

`pub(crate)` 或 `pub(in crate::a)` 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：

- `pub` 表示可见性无限制
- `pub(crate)` 表示在当前包可见
- `pub(self)` 在当前模块可见
- `pub(super)` 在父模块可见
- `pub(in <path>)` 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

### 8.2. 示例

```rust
// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前包中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
```