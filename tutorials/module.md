# 模块

- [模块](#模块)
  - [简介](#简介)
  - [嵌套模块](#嵌套模块)
  - [模块树](#模块树)
    - [父子模块](#父子模块)
  - [用路径引用模块](#用路径引用模块)
    - [绝对路径引用](#绝对路径引用)
    - [相对路径引用](#相对路径引用)
    - [相对还是绝对](#相对还是绝对)
  - [代码可见性](#代码可见性)
    - [pub 关键字](#pub-关键字)
  - [super 引用模块](#super-引用模块)
  - [self 引用模块](#self-引用模块)
  - [结构体和枚举的可见性](#结构体和枚举的可见性)
  - [模块与文件分离](#模块与文件分离)

2023-10-30, 16:04
@author Jiawei Mao
****

## 简介

模块是 Rust 的代码构成单元。使用模块可以将包中的代码按照功能进行重组，提高可读性及易用性。同时，还能灵活地控制代码的可见性，进一步强化 Rust 的安全性。

## 嵌套模块

使用 `cargo new --lib restaurant` 创建一个小餐馆，注意，这里创建的是一个库类型的 `Package`，然后将以下代码放入 `src/lib.rs` 中：

```rust
// 餐厅前厅，用于吃饭
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

这里创建了三个模块，要点：

- 使用 `mod` 关键字创建模块，后跟模块名称
- 模块可以嵌套，这里嵌套是因为招待客人和服务都在前厅，这里模拟了真实场景
- 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
- 所有模块均定义在同一个文件中

使用模块能将功能相关的代码组织到一起，然后通过模块名称来说明这些代码为何被组织在一起。这样其它程序员在使用你的模块时，就可以更快地理解和上手。

## 模块树

`src/main.rs` 和 `src/lib.rs` 被称为包根（crate root），这两个文件的内容形成了一个模块 `crate`，该模块位于包的树形结构(由模块组成的树形结构)的根部：

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

这颗树展示了模块之间的嵌套关系，因此被称为**模块树**。其中 `crate` 包根是 `src/lib.rs` 文件，包根文件中的三个模块分别形成了模块树的剩余部分。

### 父子模块

如果模块 `A` 包含模块 `B`，那么 `A` 是 `B` 的父模块，`B` 是 `A` 的子模块。在上例中，`front_of_house` 是 `hosting` 和 `serving` 的父模块，反之，后两者是前者的子模块。

模块树跟计算机文件系统目录树类似，使用方式也很相似：每个文件都有自己的路径，用户可以通过这些路径使用它们，在 Rust 中，我们也通过路径的方式来引用模块。

## 用路径引用模块

想要调用一个函数，需要知道它的路径，Rust 中有两种路径：

- **绝对路径**，从包根开始，路径名以包名或者 `crate` 开头
- **相对路径**，从当前模块开始，以 `self`，`super` 或当前模块的标识符开头

继续以小餐馆，这次为它实现一个小功能，修改文件 `src/lib.rs`：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

上面的代码省去了其余模块和函数，关注在函数调用上。`eat_at_restaurant` 函数定义在包根中，在该函数中使用了两种方式调用 `add_to_waitlist`。

### 绝对路径引用

因为 `eat_at_restaurant` 和 `add_to_waitlist` 定义在一个包中，因此在绝对路径引用时只需以 `crate` 开头，然后逐层引用，每一层之间用 `::` 分隔：

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

对比模块树：

```
crate
 └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

可以看出，绝对路径的调用符合模块树的层级递进。

### 相对路径引用

回到模块树中，因为 `eat_at_restaurant` 和 `front_of_house` 都处于包根 `crate` 中，因此相对路径可以使用 `front_of_house` 作为开头：

```rust
front_of_house::hosting::add_to_waitlist();
```

### 相对还是绝对

如果只是为了引用指定模块中的对象，两种方式都可以。在实际使用时，需要遵循一个原则：**当代码被挪动位置时，尽量减少引用路径的修改**。

回到之前的例子，如果把 `front_of_house` 模块和 `eat_at_restaurant` 移动到 `customer_experience` 模块中，那么绝对路径的引用方式必须修改为：`crate::customer_experience::front_of_house ...`，而相对路径的方式无需修改，因为它们两个的相对位置没有变：

```
crate
 └── customer_experience
    └── eat_at_restaurant
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
```

再比如，其它的都不动，把 `eat_at_restaurant` 移动到模块 `dining` 中，如果使用相对路径，你需要修改该路径，但如果使用的是绝对路径，就无需修改：

```
crate
 └── dining
     └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
```

如果不确定哪个好，可以考虑**优先使用绝对路径**，因为调用的地方和定义的地方往往是分离的，而定义的地方较少会变动。

## 代码可见性

运行下面的代码：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

报错啦:

```sh
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
```

错误信息：`hosting` 是私有模块，无法在包根访问，那么为何 front_of_house 模块就可以访问？因为它和 `eat_at_restaurant` 同属于一个包根作用域内，同一个模块内的代码自然不存在私有化问题(所以我们之前章节的代码都没有报过这个错误！)。

模块除了用于组织代码，还能定义代码的私有化边界，如果希望让函数或者结构体等类型变成私有化，可以使用模块。

Rust 出于安全的考虑，所有的类型默认都是私有化的，包括函数、方法、结构体、枚举、常量，就连模块本身也是私有化的。在 Rust 中，父模块无法访问子模块中的私有项，但是子模块却可以访问父模块、父父..模块的私有项。

### pub 关键字

类似其它语言的 `public`，Rust 提供了 `pub` 关键字，通过它可以控制模块和模块中指定项的可见性。

前面的错误只需要将 `hosting` 模块标记为对外可见即可：

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

/*--- snip ----*/
```

但是，又报错了：

```sh
error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
```

模块可见还不够，还需要将函数 `add_to_waitlist` 标记为可见？模块可见性不代表模块内部项的可见性，模块的可见性仅仅是允许其它模块去引用它，但是想要引用它内部的项，还得继续将对应的项标记为 `pub`。

既然知道了如何解决，那么为函数也标记上 pub：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*--- snip ----*/
```

编译顺利通过。

## super 引用模块

`super` 表示以父模块为起点的引用方式，类似于文件系统中的 `..` 语法，`src/lib.rs` 文件：

```rust
fn serve_order() {}

// 厨房模块
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

在厨房模块中，使用 `super::serve_order` 调用父模块(包根)中的 `serve_order` 函数。

你可能会问，为何不使用 `crate::serve_order` ？额，其实也可以，不过如果你确定未来这种层级关系不会改变，那么 `super::serve_order` 的方式会更稳定，未来就算它们都不在包根了，依然无需修改引用路径。所以路径的选用，往往还是取决于场景，以及未来代码的可能走向。

## self 引用模块

self 引用自身模块中的项：

```rust
fn serve_order() {
    self::back_of_house::cook_order()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::serve_order();
    }

    pub fn cook_order() {}
}
```

这里 `self` 可以去掉，完全可以直接调用 `back_of_house`，但是 `self` 还有一个大用处，后面会讲。

## 结构体和枚举的可见性

结构体和枚举的成员字段拥有完全不同的可见性：

- 将结构体设置为 `pub`，但它的所有字段依然是私有的
- 将枚举设置为 `pub`，它的所有字段也将对外可见

原因在于，枚举和结构体的使用方式不一样。如果枚举的成员对外不可见，那该枚举将一点用都没有，因此枚举成员的可见性自动跟枚举可见性保持一致，这样可以简化用户的使用。

而结构体的应用场景比较复杂，其中的字段也往往部分在 A 处被使用，部分在 B 处被使用，因此无法确定成员的可见性，那索性就设置为全部不可见，将选择权交给程序员。

## 模块与文件分离

在之前的例子中，所有模块都定义在 `src/lib.rs` 中，但是当模块变多或者变大时，需要将模块放入一个单独的文件中，让代码更好维护。

现在，把 `front_of_house` 前厅分离出来，放入一个单独的文件中 `src/front_of_house.rs`：

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

然后，将以下代码留在 `src/lib.rs` 中：

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

要点：

- `mod front_of_house;` 告诉 Rust 从一个和模块 `front_of_house` 同名的文件中加载该模块的内容
- 使用绝对路径的方式来引用 `hosting` 模块：`crate::front_of_house::hosting;`

需要注意的是，和之前的 `mod front_of_house{..}` 的完整模块不同，现在模块的声明和实现是分离的，实现在单独的 `front_of_house.rs` 文件中，然后通过 `mod front_of_house;` 这条声明语句从该文件中把模块内容加载进来。因此我们可以认为，模块 `front_of_house` 的定义还是在 `src/lib.rs` 中，只不过模块的具体内容被移动到了 `src/front_of_house.rs` 文件中。

在这里出现了一个新的关键字 `use`，该关键字用来将外部模块中的项引入到当前作用域中来，这样无需冗长的父模块前缀即可调用：`hosting::add_to_waitlist();`。

当一个模块有许多子模块时，也可以通过文件夹的方式来组织这些子模块。

在上述例子中，可以创建一个目录 `front_of_house`，然后在文件夹里创建一个 `hosting.rs` 文件，`hosting.rs` 文件只剩下：

```rust
pub fn add_to_waitlist() {}
```

编译，报错：

```sh
error[E0583]: file not found for module `front_of_house`
 --> src/lib.rs:3:1
  |
1 | mod front_of_house;
  | ^^^^^^^^^^^^^^^^^^
  |
  = help: to create the module `front_of_house`, create file "src/front_of_house.rs" or "src/front_of_house/mod.rs"
```

如果将文件夹作为模块，需要显示指定暴露哪些子模块。按照上述的报错信息，有两种解决方法：

- 在 `front_of_house` 目录里创建一个 `mod.rs`，如果你使用的 rustc 版本 1.30 之前，这是唯一的方法。
- 在 `front_of_house` 同级目录里创建一个与模块（目录）同名的 rs 文件 `front_of_house.rs`，在新版本里，建议使用这样的命名方式来避免项目中存在大量同名的 `mod.rs` 文件。

而无论是上述哪个方式创建的文件，其内容都是一样的，你需要定义你的子模块（子模块名与文件名相同）：

```rust
pub mod hosting;
// pub mod serving;
```