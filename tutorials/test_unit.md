# 单元测试

- [单元测试](#单元测试)
  - [简介](#简介)
  - [条件编译 cfg(test)](#条件编译-cfgtest)
  - [测试私有函数](#测试私有函数)

Last updated: 2023-10-17, 10:04
@author Jiawei Mao
****

## 简介

单元测试目标是测试某一个代码单元(一般是函数)是否能按照预期进行工作。例如，测试一个 `add` 函数，验证当给予两个输入时，最终返回的结果是否符合预期。

在 Rust 中，单元测试的惯例是**将测试代码的模块跟待测试的正常代码放入同一个文件中**，例如 src/lib.rs 文件中有如下代码:

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
- 同一个文件中包含测试模块 `tests`，并且使用 `#[cfg(test)]` 进行标注

## 条件编译 cfg(test)

`#[cfg(test)]` 标注告诉 Rust 只在 `cargo test` 时才编译和运行模块 `tests`，其它时候当这段代码是空气即可。这么做有几个好处：

- 节省构建代码时的编译时间
- 减小编译出的可执行文件的体积

集成测试需要这个标注，因为它们被放入单独的目录文件中；而单元测试跟正常的逻辑代码在同一个文件，因此必须对其进行特殊的标注，以供 Rust 识别。

在 `#[cfg(test)]` 中， `cfg` 是配置 configuration 的缩写，它告诉 Rust ：当 test 配置项存在
时，才运行下面的代码，而 `cargo test` 在运行时，就会将 `test` 这个配置项传入进来，因此后面
的 `tests` 模块会被包含进来。

这就是典型的条件编译。

## 测试私有函数

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

`internal_adder` 没有使用 `pub` 进行声明，因此它是一个私有函数。`tests` 作为另一个模块，按理是无法对它进行调用的，因为它们根本不在同一个模块中！

但是在上述代码中，我们使用 `use super::*;` 将 `tests` 的父模块中的所有内容引入到当前作用域中，这样就可以实现对私有函数的测试。
