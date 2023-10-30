# 项目和包

- [项目和包](#项目和包)
  - [1. 简介](#1-简介)
  - [2. 项目 Package](#2-项目-package)
    - [2.1. binary Package](#21-binary-package)
    - [2.2. library Package](#22-library-package)
  - [3. Package 和包的区别](#3-package-和包的区别)
  - [4. Package 的典型结构](#4-package-的典型结构)

2023-10-30, 14:31
@author Jiawei Mao
****

## 1. 简介

Rust 组织管理代码的结构包括：

- **项目**（Packages）：用来构建、测试和分享包
- **包**（Crate）：由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件
- **模块**(Module)：一个文件可以包含多个模块，也可以只包含一个模块，模块是项目的代码组织单元

包会将相关的功能打包在一起，便于共享。

## 2. 项目 Package

Rust 的 Package 和其它编程语言的**项目**对应，可以理解为软件包。

`Package` 是一个项目，它包含独立的 `Cargo.toml` 文件，并包含至少一个包。一个项目可以包含：

- 一个 `library` 类型的包
- 多个可执行 `binary` 类型的包

### 2.1. binary Package

创建 binary 项目：

```sh
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

说明：

- `Cargo` 创建了一个名为 `my-project` 的项目，并在其中创建了 `Cargo.toml` 文件
- `Cargo.toml` 没有指定 `src/main.rs` 为程序入口，但 `Cargo` 默认将 `src/main.rs` 作为 binary 包的根文件：
  - 即创建 `my-project` 项目时创建了同名的 binary 包
  - 所有代码从 `src/main.rs` 文件中的 `fn main()` 函数开始执行

使用 `cargo run` 可以运行该项目，输出：`Hello, world!`。

### 2.2. library Package

创建 `library` 类型的项目：

```sh
$ cargo new my-lib --lib
     Created library `my-lib` package
$ ls my-lib
Cargo.toml
src
$ ls my-lib/src
lib.rs
```

运行 my-lib 报错：

```sh
$ cargo run
error: a bin target must be available for `cargo run`
```

`library` 类型的`项目`只能被其它项目引用，不能独立运行。

与 `src/main.rs` 一样，`Cargo` 对包含 `src/lib.rs` 文件的项目自动创建一个同名的 library 类型包 `my-lib`，该包的根文件是 `src/lib.rs`。

## 3. Package 和包的区别

之所以容易混淆 Package 和包，是因为 `cargo new` 创建的 Pacakge 和它包含的包同名。

不过 Package 是一个项目，而包只是一个编译单元：`src/main.rs` 和 `src/lib.rs` 都是编译单元，也就是包。

## 4. Package 的典型结构

上面创建的 `Package` 仅包含` src/main.rs` 文件，表示它仅包含一个 binary 同名包 `my-project`。

如果一个 `Package` 同时包含 `src/main.rs` 和 `src/lib.rs`，表示它包含两个包：`library` 包和 `binary` 包，且包名都是 `my-project`。

真实的 `Package` 可能包含多个 `binary` 包，这些包文件放在 `src/bin` 目录，每一个文件对应一个独立的 binary 包，同时也会包含一个 library 包，该包只能有一个 `src/lib.rs`：

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

说明：

- 唯一 library 包：`src/lib.rs`
- 默认 binary 包：`src/main.rs`，编译后生成的可执行文件与 `Package` 同名
- 其余 binary 包：`src/bin/main1.rs` 和 `src/bin/main2.rs`，它们会分别生成一个与文件同名的 binary 可执行文件
- 集成测试文件：`tests` 目录下
- 基准性能测试 `benchmark` 文件：`benches` 目录下
- 项目示例：`examples` 目录下

这基本上是 Rust 的标准目录结构，在 GitHub 的大多数项目上都与此类似。
