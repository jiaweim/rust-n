# Cargo

- [Cargo](#cargo)
  - [简介](#简介)
  - [构建运行](#构建运行)
  - [构建发布](#构建发布)
  - [包软件](#包软件)
  - [Cargo.toml 和 Cargo.lock](#cargotoml-和-cargolock)
  - [配置 crates.io 镜像](#配置-cratesio-镜像)
    - [新增镜像地址](#新增镜像地址)
    - [覆盖默认镜像](#覆盖默认镜像)
  - [依赖项](#依赖项)

Last updated: 2023-10-16, 13:05
@author Jiawei Mao
****

## 简介

概要：

- cargo 是 rust 的编译管理器、包管理器和通用工具。
- rustc 是 Rust 编译器，通常不需要直接调用，而通过 cargo 间接调用。
- rustdoc 是 Rust 文档工具。

## 构建运行

- `cargo new`：创建项目
- `cargo build`：构建项目
- `cargo run`：构建并运行项目
- `cargo check`：检查是否能够编译

**不设置 git**

```sh
cargo new hello --vcs none
```

**构建并运行项目**

```sh
cargo run
```

Cargo 先调用编译器 rustc，然后运行它生成的可执行文件。Cargo 将可执行文件放在 `target` 目录中。

## 构建发布

`cargo build --release`

添加 `--release` 会优化代码，使 Rust 代码运行更快，但会延长编译时间。

## 包软件

`cargo update`

根据 Cargo.toml 文件更新 Cargo.lock 文件中的包的版本。

## Cargo.toml 和 Cargo.lock

`Cargo.toml` 和 `Cargo.lock` 是 cargo 的核心文件，它的所有活动均基于这两个文件。

- `Cargo.toml` 是 cargo 的**项目数据描述文件**。它存储了项目的所有元配置信息，如果Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 `Cargo.toml`。
- `Cargo.lock` 文件是 cargo 工具根据同一项目的 `toml` 文件生成的**项目依赖详细清单**，因此我们一般不用修改它，只需要对着 `Cargo.toml` 文件撸就行了。

!!! note
    什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock ，如果是一个依赖库项目，那么请把它添加到 .gitignore 中。



## 配置 crates.io 镜像

`crates.io` 的默认镜像地址在国外，因此下载依赖项会下载缓慢甚至卡住。

为了使用 crates.io 之外的注册服务，需要对 `$HOME/.cargo/config.toml` 文件进行配置，添加新的服务提供商。实现方式有两种：

- 增加新的镜像地址
- 覆盖默认镜像地址

### 新增镜像地址

找到 `$HOME/.cargo/config.toml` 文件（如果没有，手动创建一个），在其中添加如下内容：

```toml
[registries]
ustc = { index = "https://mirrors.ustc.edu.cn/crates.io-index/" }
```

这种方式新增一个镜像地址，在引入依赖时，需要指定该地址。例如在项目中引入 `time` 包，需要在 Cargo.toml 中使用如下方式：

```toml
[dependencies]
time = {  registry = "ustc" }
```

重新配置后，初次构建可能需要很长时间，因为需要下载 ustc 注册服务的索引文件。

上面使用的是科大镜像，也是国内最早的 Rust 注册服务。其它镜像还有：

- **字节跳动**

这个镜像不限速，设置方式：

```toml
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

# 稀疏索引，要求 cargo >= 1.68
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

### 覆盖默认镜像

新增镜像，在添加依赖项时需要指定地址，比较麻烦。直接使用新服务替代默认的 crates.io 更方便。

在 `$HOME/.cargo/config.toml` 添加以下内容：

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

这里创建了一个新的镜像源 `[source.ustc]`，然后将默认的 crates.io 替换为新的镜像源 `replace-with = 'ustc'`。

## 依赖项

在 Cargo.toml 中，支持三种依赖项：

- 基于 Rust 官方仓库 crates.io，通过版本说明来描述
- 基于项目源代码的 git 仓库地址，通过 URL 来描述
- 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述

三种形式：

```toml
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```

**crates.io 语法**

```rust
[dependencies]
time = "0.1.12"
```

^ 指定版本：

```
^1.2.3  :=  >=1.2.3, <2.0.0
^1.2    :=  >=1.2.0, <2.0.0
^1      :=  >=1.0.0, <2.0.0
^0.2.3  :=  >=0.2.3, <0.3.0
^0.2    :=  >=0.2.0, <0.3.0
^0.0.3  :=  >=0.0.3, <0.0.4
^0.0    :=  >=0.0.0, <0.1.0
^0      :=  >=0.0.0, <1.0.0
```

