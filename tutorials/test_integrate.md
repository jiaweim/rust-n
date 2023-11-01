# 集成测试

- [集成测试](#集成测试)
  - [1. 简介](#1-简介)
  - [2. tests 目录](#2-tests-目录)
  - [3. 测试过滤](#3-测试过滤)
  - [4. 共享模块](#4-共享模块)

2023-11-01, 21:27
@author Jiawei Mao
****

## 1. 简介

与单元测试不同，集成测试的代码在一个单独的目录中。由于它们使用跟其它模块一样的方式调用要测试的代码，因此只能调用 `pub` API。

如果说单元测试是对**代码单元**进行测试，那集成测试则是对某一个**功能或者接**口进行测试，因此单元测试通过，并不意味着集成测试能通过：局部上反映不出的问题，在全局上也许会暴露出来。

## 2. tests 目录

集成测试放在与 `src` 同级的 `tests` 目录。

在该目录可以创建任意多的测试文件，Cargo 会自动来此目录下寻找集成测试文件。在该目录下创建任何文件，Cargo 会对每个文件都进行自动编译，但友情提示下，最好按照合适的逻辑来组织你的测试代码。

首先创建一个集成测试文件 `tests/integration_test.rs`，注意，`tests` 目录一般需要手动创建，该目录在项目的根目录下，跟 `src` 目录同级。此时的目录结构：

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```


`integration_test.rs` 文件内容：

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

这段测试代码是对之前私有函数中的示例进行测试，该示例代码在 `src/lib.rs` 中。

- 与单元测试有所不同，这里并没有创建测试模块
- `tests` 目录下的测试文件都是独立的 crate，因此需要在每个文件导入库 `use adder`

创建项目后， `src/lib.rs` 自动创建一个与项目同名的 lib 类型包，由于项目名是 `adder` ，因此包名也是 `adder` 。

因为 `tests` 目录本身就说明了它的特殊用途，因此无需再使用 `#[cfg(test)]` 标注测试。Cargo 在运行 `cargo test` 时会对 `tests` 目录中的每个文件进行编译运行。

```sh
$ cargo test
     Running unittests (target/debug/deps/adder-8a400aa2b5212836)

running 1 test
test tests::it_works ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-
2d3aeee6f15d1f20)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s
```

运行 `cargo test`，可以看到以上输出。测试内容有三个部分：单元测试，集成测试和文档测试。

- 首先运行单元测试 `Running unittests`
- 然后运行集成测试 `Running tests/integration_test.rs`，可以看出，集成测试的输出内容与单元测试并没有大的区别。
- 最后运行的是文档测试 `Doc-tests adder`。

## 3. 测试过滤

与单元测试类似，可以通过**指定名称运行特定集成测试** `cargo test --test 集成测试文件名`：

```sh
$ cargo test --test integration_test
     Running tests/integration_test.rs (target/debug/deps/integration_test-
82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s
```

此时，单元测试、文档测试啥的都没有运行，只有集成测试目录下的 `integration_test` 文件被顺利执行。

## 4. 共享模块

在集成测试的 `tests` 目录下，每一个文件都是一个独立的包，这种组织方式可以帮助理清测试代码的关系，但是如果大家想要在多个文件中共享同一个功能该怎么做？例如函数 `setup` 可以用于状态初始化，然后多个测试包都需要使用该函数进行状态的初始化。

`tests` 目录下的子目录不会被当做测试的包，也不会有测试输出，可以在其中执行初始化工作。

例如，创建 `tests/common/mod.rs`，在其中添加辅助测试函数，此时的项目结构：

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

将 `setup` 函数移到 `tests/common/mod.rs`，测试中就不会出现这部分。

然后就能在其它测试中调用 setup 函数。

**示例：** tests/integration_test.rs

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
