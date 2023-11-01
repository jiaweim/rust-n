# 单元测试

- [单元测试](#单元测试)
  - [1. 简介](#1-简介)
  - [2. 条件编译 cfg(test)](#2-条件编译-cfgtest)
  - [3. 测试私有函数](#3-测试私有函数)

Last updated: 2023-10-17, 10:04
@author Jiawei Mao
****

## 1. 简介

单元测试的目标是测试某一个代码单元(一般是函数)是否能按照预期进行工作。例如，测试一个 `add` 函数，验证当给予两个输入时，最终返回的结果是否符合预期。

在 Rust 中，习惯将**将单元测试和待测试的代码放在同一个文件**，例如 `src/lib.rs` 文件中有如下代码:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
```

其中：

- `add_two` 是项目代码
- 同一个文件中包含测试模块 `tests`，使用 `#[cfg(test)]` 进行标注

## 2. 条件编译 cfg(test)

`#[cfg(test)]` 注解告诉 Rust 只在执行 `cargo test` 时才编译和运行测试代码。这么做有几个好处：

- 节省编译时间
- 减小编译出的可执行文件大小

集成测试位于单独的文件夹，不需要这个标注；单元测试跟正常的逻辑代码在同一个文件，必须对其进行特殊的标注，以供 Rust 识别。

在 `#[cfg(test)]` 中， `cfg` 属性表示配置（configuration），它告诉 Rust 只有 test 配置项存在时才运行下面的代码，而 `cargo test` 在运行时会将 `test` 配置项传入进来，因此后面的 `tests` 模块会被包含进来。

这是典型的条件编译。

## 3. 测试私有函数

Rust 支持测试私有函数：

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

`internal_adder` 没有标记为 `pub`，它是私有函数。上述代码使用 `use super::*;` 将 `tests` 的父模块的所有内容引入到当前作用域，这样就可以访问私有函数。
