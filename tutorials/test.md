# 编写测试和控制执行

- [编写测试和控制执行](#编写测试和控制执行)
  - [1. 简介](#1-简介)
  - [2. 测试函数](#2-测试函数)
  - [3. 自定义失败信息](#3-自定义失败信息)
  - [4. 测试 panic](#4-测试-panic)
  - [5. 使用 Result](#5-使用-result)
  - [6. 使用 `--` 分割命令行参数](#6-使用----分割命令行参数)
  - [7. 测试函数中的 println!](#7-测试函数中的-println)
  - [8. 运行部分测试](#8-运行部分测试)
    - [8.1. 运行单个测试](#81-运行单个测试)
    - [8.2. 指定名称的一部分过滤测试](#82-指定名称的一部分过滤测试)
    - [8.3. 通过模块名称过滤测试](#83-通过模块名称过滤测试)
    - [8.4. 忽略测试](#84-忽略测试)
    - [8.5. 组合过滤](#85-组合过滤)
  - [9. dev-dependencies](#9-dev-dependencies)

Last updated: 2023-10-17, 09:48
@author Jiawei Mao
****

## 1. 简介

测试函数一般执行三种行为：

1. 设置数据或状态
2. 运行想要测试的代码
3. 判断返回的结果是否符合预期

## 2. 测试函数

当使用 Cargo 创建 `lib` 类型的包时，会自动创建一个测试模块。此时 `lib.rs` 的内容如下：

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

其中：

- `tests` 是测试模块，由 `#[cfg(test)]` 标记
- `it_works` 是测试函数，由 `#[test]` 标记
- `assert_eq!` 是内置断言

使用 `cargo test` 运行项目中的所有测试。

## 3. 自定义失败信息

`assert!`, `assert_eq!` 和 `assert_ne!` 宏传递一个可选的失败信息参数，在测试失败时会将自定义失败信息一起打印出来。

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("god");
        let target = "关羽";
        assert!(
            result.contains(target),
            "你的问候中并没有包含目标姓名 {} ，你的问候是 `{}`",
            target,
            result
        );
    }
}
```

## 4. 测试 panic

- 使用 `should_panic` 属性测试函数是否会 panic

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

- 使用 `expected` 参数指定 panic 信息

`expected` 字符串与 panic 内容不需要完全相同，只需要是后者的前缀即可。

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## 5. 使用 Result

使用 `Result<T, E>` 重写 `it_works()` 测试，在失败时返回 `Err` 而非 panic：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

`it_works` 函数的返回值类型为 `Result<(), String>`，与 `assert_eq!` 宏不同，该测试在成功时返回 `Ok(())`，测试失败时返回带 `String` 的 `Err`。

对使用 `Result<T, E>` 的测试不能使用 `#[should_panic]` 注解。为了断言一个操作返回 Err，不要对 `Result<T, E>` 使用 ? 表达式，而是使用 `assert!(value.is_err())`。

## 6. 使用 `--` 分割命令行参数

`cargo build` 可以将代码编译成一个可执行文件，那么 `cargo run` 和 `cargo test` 是如何运行的吗？其实道理都一样，这两个也是将代码编译成可执行文件，然后进行运行，唯一的区别就在于这个可执行文件随后会被删除。

`cargo test` 在测试模式下编译代码并运行生成的二进制文件：

- `cargo test` 生成的二进制文件默认并发运行所有测试
- 截获测试过程中产生的输出，阻止它们显示

可以将一部分命令行参数传递给 `cargo test`，而将另一部分传递给生成的测试二进制文件。这两种参数使用 `--` 分割：

1. 提供给 `cargo test` 的参数在 `--` 前面
2. 提供给编译后的可执行文件的参数在 `--` 后面

可以使用 `cargo test --help` 查看第一种参数的帮助列表；使用 `cargo test -- --help` 查看第二种参数的帮助列表。

**示例：** 单线程运行测试

```sh
$ cargo test -- --test-threads=1
```

## 7. 测试函数中的 println!

默认情况下，如果测试通过，写入标准输出的内容不会在测试结果中显示。如果想看所有的输出，使用命令：

```sh
$ cargo test -- --show-output
```

## 8. 运行部分测试

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

### 8.1. 运行单个测试

指定测试函数名作为参数：

```sh
$ cargo test one_hundred
```

### 8.2. 指定名称的一部分过滤测试

```sh
$ cargo test add

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out;
finished in 0.00s
```

这里运行了带 `add` 的所有测试。

- 不仅可以使用前缀，还能使用名称中间部分过滤测试

```sh
$ cargo test and

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; 
finished in 0.00s
```

### 8.3. 通过模块名称过滤测试

```sh
$ cargo test tests

running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s
```

### 8.4. 忽略测试

通过 `ignore` 属性忽略特定测试：

```rust
 #[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 这里的代码需要几十秒甚至几分钟才能完成
}
```

这里用 `#[ignore]` 对 `expensive_test` 函数进行了标注，看看结果：

```sh
$ cargo test
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; 
finished in 0.00s

   Doc-tests adder
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s
```

输出中的 `test expensive_test ... ignored` 表示该测试函数被忽略了，没有被执行。

- 使用 `cargo test -- --ignored` **仅运行**被忽略的测试

```sh
$ cargo test -- --ignored
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; 
finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; 
finished in 0.00s
```

### 8.5. 组合过滤

组合上面介绍的过滤方式，更加强大。以如下代码为例：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 这里的代码需要几十秒甚至几分钟才能完成
    }

    #[test]
    #[ignore]
    fn expensive_run() {
        // 这里的代码需要几十秒甚至几分钟才能完成
    }
}
```

- 运行 `tests` 模块中被忽略的测试函数

```sh
$ cargo test tests -- --ignored
running 2 tests
test tests::expensive_test ... ok
test tests::expensive_run ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; 
finished in 0.00s
```

- 运行名称中带 run 且被忽略的测试

```sh
$ cargo test run -- --ignored
running 1 test
test tests::expensive_run ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; 
finished in 0.00s
```

## 9. dev-dependencies

`[dev-dependencies]` 用于引入只在测试场景使用的外部依赖。

例如，`pretty_assertions` 库扩展标准库中的 `assert_eq!` 和 `assert_ne!`，并提供彩色字体的结果对比。

在 `Cargo.toml` 文件中引入 `pretty_assertions`：

```toml
# standard crate data is left out
[dev-dependencies]
pretty_assertions = "1"
```

然后在 `src/lib.rs` 中使用：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // 该包仅能用于测试

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); 
```
