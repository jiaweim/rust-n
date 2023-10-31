# 模块

- [模块](#模块)
  - [1. 简介](#1-简介)
  - [2. 嵌套模块](#2-嵌套模块)
  - [3. 模块树](#3-模块树)
  - [4. 引用模块](#4-引用模块)
    - [4.1. 绝对引用](#41-绝对引用)
    - [4.2. 相对引用](#42-相对引用)
    - [4.3. 相对还是绝对](#43-相对还是绝对)
  - [5. 代码可见性](#5-代码可见性)
    - [5.1. pub 关键字](#51-pub-关键字)
  - [6. super 引用模块](#6-super-引用模块)
  - [7. self 引用模块](#7-self-引用模块)
  - [8. 结构体和枚举的可见性](#8-结构体和枚举的可见性)
  - [9. 模块与文件分离](#9-模块与文件分离)
    - [9.1. 子文件夹](#91-子文件夹)

2023-10-30, 16:04
@author Jiawei Mao
****

## 1. 简介

模块是 Rust 的代码构成单元。使用模块可以将包中的代码按照功能进行分组，提高可读性和易用性。同时还能灵活地控制代码的可见性，进一步强化 Rust 的安全性。

## 2. 嵌套模块

使用 `cargo new --lib restaurant` 创建一个餐馆，注意，这里创建的是一个 `lib` 类型的 `Package`，然后将以下代码放入 `src/lib.rs`：

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
- 模块可以嵌套
- 模块中可以定义各种 Rust 类型，如函数、结构体、枚举、特征等
- 所有模块定义在同一个文件中

使用模块能将功能相关的代码组织到一起，然后通过模块名称来说明这些代码为何被组织在一起。这样其他程序员在使用该模块时，能够更快地理解和上手。

## 3. 模块树

`src/main.rs` 和 `src/lib.rs` 被称为 crate-root，这两个文件的内容形成了一个模块 `crate`，该模块位于由模块组成的树形结构的根部：

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

这颗树展示了模块之间的嵌套关系，因此被称为**模块树**：

- crate-root 文件为 `crate`，这里是 `src/lib.rs` 文件
- crate-root 文件中的三个模块形成了模块树的剩余部分

**父子模块**

如果模块 `A` 包含模块 `B`，那么 `A` 是 `B` 的父模块，`B` 是 `A` 的子模块。在上例中，`front_of_house` 是 `hosting` 和 `serving` 的父模块，而后两者是前者的子模块。

模块树跟计算机文件系统目录树类似，使用方式也很相似：每个文件都有自己的路径，用户可以通过这些路径使用文件；Rust 也通过路径来引用模块。

## 4. 引用模块

调用一函数需要知道它的路径，Rust 有两种路径：

- **绝对路径**，从 crate-root 开始，路径名以包名或 `crate` 开头
- **相对路径**，从当前模块开始，以 `self`，`super` 或当前模块的标识符开头

继续以餐馆为例，这次为它实现一个功能，修改文件 `src/lib.rs` 为：

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

这里略掉了其它模块和函数，只关注函数调用。`eat_at_restaurant` 函数定义在 crate-root 中，在该函数中使用了两种方式调用 `add_to_waitlist`。

### 4.1. 绝对引用

由于 `eat_at_restaurant` 和 `add_to_waitlist` 定义在一个包中，因此绝对路径引用只需以 `crate` 开头，然后逐层引用，层与层之间用 `::` 分隔：

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

### 4.2. 相对引用

在模块树中 `eat_at_restaurant` 和 `front_of_house` 都处于 crate-root `crate` 中，因此相对路径可以直接以 `front_of_house` 开头：

```rust
front_of_house::hosting::add_to_waitlist();
```

### 4.3. 相对还是绝对

如果只是引用指定模块中的对象，两种方式都可以。在实际使用时，尽量遵循一个原则：**当挪动代码时，尽量减少引用路径的修改**。

回到之前的例子，如果把 `front_of_house` 模块和 `eat_at_restaurant` 移到 `customer_experience` 模块，那么绝对引用需要修改为：`crate::customer_experience::front_of_house ...`，而相对路径无需修改，因为它们两个的相对位置没有变化：

```
crate
 └── customer_experience
    └── eat_at_restaurant
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
```

再比如，其它不动，把 `eat_at_restaurant` 移动到 `dining` 模块，如果使用相对路径，则需要修改该路径，使用绝对路径就无需修改：

```
crate
 └── dining
     └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
```

如果选择困难，可以**优先使用绝对路径**，因为调用的地方和定义的地方往往是分离的，而定义的地方较少会变动。

## 5. 代码可见性

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

错误信息：`hosting` 是私有模块，无法在 crate-root 访问。

那么为何能访问 `front_of_house` 模块？因为它和 `eat_at_restaurant` 同属一个 crate-root 作用域，同一个模块内的代码不存在私有化问题。

!!! info
    模块除了用于组织代码，还能定义代码的私有化边界，如果希望让函数或者结构体等类型变成私有化，可以使用模块。
    
    Rust 出于安全考虑，所有的类型**默认私有化**，包括函数、方法、结构体、枚举、常量，就连模块本身也是私有化的。在 Rust 中，父模块无法访问子模块中的私有项，但是子模块却可以访问父模块、父父..模块的私有项。

### 5.1. pub 关键字

类似其它语言的 `public`，Rust 提供了 `pub` 关键字控制模块和模块中指定项的可见性。

前面的错误只需要将 `hosting` 模块标记为 `pub` 即可：

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

模块可见还不够，还需要将函数 `add_to_waitlist` 标记为可见。

模块可见性不代表模块内部项的可见性，模块的可见性仅仅是允许其它模块引用它，但是想要引用它内部的项，还得继续将对应的项标记为 `pub`。

将函数也标记为 `pub`：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*--- snip ----*/
```

编译顺利通过。

## 6. super 引用模块

`super` 表示以父模块为引用起点，类似于文件系统中的 `..` 语法，`src/lib.rs` 文件：

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

在 `back_of_house` 模块中，使用 `super::serve_order` 调用父模块(crate-root)中的 `serve_order` 函数。

你可能会问，为何不使用 `crate::serve_order` ？其实也可以，不过如果确定以后这种层级关系不会改变，那么 `super::serve_order` 的方式会更稳定，就算它们都不在 crate-root 了，依然无需修改引用路径。所以路径的选用，往往还是取决于场景，以及未来代码的可能走向。

## 7. self 引用模块

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

这里 `self` 引用 `crate` 模块，其实可以去掉，完全可以直接调用 `back_of_house`，但是 `self` 还有其它用处，后面会讲。

## 8. 结构体和枚举的可见性

struct 和 enum 的成员字段拥有不同的可见性：

- 将结构体设置为 `pub`，它的所有字段依然是私有的
- 将枚举设置为 `pub`，它的所有字段也成为 `pub`

原因在于，枚举和结构体的使用方式不一样。如果枚举的成员对外不可见，那该枚举将毫无用处，因此枚举成员的可见性自动跟枚举可见性保持一致。

而结构体的应用场景比较复杂，其中的字段也往往部分在 A 处被使用，部分在 B 处被使用，因此无法确定成员的可见性，那索性就设置为全部不可见，将选择权交给程序员。

## 9. 模块与文件分离

在之前的例子中，所有模块都定义在 `src/lib.rs` 中，当模块变多或者变大时，需要将模块放入单独的文件，让代码更好维护。

现在，把 `front_of_house` 分离出来，放入文件 `src/front_of_house.rs`：

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

- `mod front_of_house;` 从一个和模块 `front_of_house` 同名的文件中加载该模块的内容
- 使用绝对路径的方式引用 `hosting` 模块：`crate::front_of_house::hosting;`

需要注意的是，这里模块的声明和实现是分离的：

- 在 `front_of_house.rs` 文件中实现
- 通过 `mod front_of_house;` 这条声明语句从 `front_of_house.rs` 文件加载模块内容

在这里出现了一个新的关键字 `use`，该关键字将外部模块的项引入到当前作用域，这样调用就无需冗长的父模块前缀：`hosting::add_to_waitlist();`。

### 9.1. 子文件夹

当一个模块有许多子模块时，也可以使用文件夹来组织这些子模块。

在上例中，可以创建一个目录 `front_of_house`，然后在文件夹里创建一个 `hosting.rs` 文件，`hosting.rs` 文件只剩下：

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

将文件夹作为模块时，需要指定暴露哪些子模块。根据报错信息，有两种解决方法：

- 在 `front_of_house` 目录里创建一个 `mod.rs`，这是 rustc 1.30 之前唯一的方法。
- 在 `front_of_house` 同级目录里创建一个与模块（目录）同名的 rs 文件 `front_of_house.rs`，在新版里建议使用该方式来避免项目中出现大量同名 `mod.rs` 文件。

无论采取哪种方式，文件内容都是一样的，即在 `mod.rs` 或 `front_of_house.rs` 中需要定义子模块（子模块名与文件名相同），即 `front_of_house.rs` 文件内容为：

```rust
pub mod hosting;
// pub mod serving;
```

!!! summary
    每个子文件夹，对应一个同名的 `*.rs` 文件，在文件中指定子文件夹包含的模块。