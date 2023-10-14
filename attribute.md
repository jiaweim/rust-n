# 属性

## 简介

属性是应用于模块、crate或项的元数据。

- 当属性作用于整个 crate 时，语法为

```rust
#![crate_attribute]
```

- 当属性作用域模块或项时，语法为

```rust
#[item_attribute]
```

少了感叹号 `!`。

- 属性可以接受参数，有不同的语法形式

```rust
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
```

- 属性可以包含多个值，可以分开到多行

```rust
#[attribute(value, value2)]
#[attribute(value, value2, value3,
            value4, value5)]
```

## dead_code

编译器提供了 `dead_code` 提示，对未使用的函数产生警告。可以用属性来禁用该提示：

```rust
fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

// 这里会生成未使用代码警告
fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
```

## crate

## cfg

条件编译可能通过两种不同的操作符实现：

- `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
- `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

两种形式使用的参数语法都相同。

```rust
// 这 个 函 数 仅 当 目 标 系 统 是 Linux 的 时 候 才 会 编 译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}
// 而 这 个 函 数 仅 当 目 标 系 统 **不是** Linux 时 才 会 编 译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}
fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

### 自定义条件

部分条件如 `target_os` 是由 `rustc` 提供的，自定义条件则必须使用 `--cfg` 标记来传给 rustc 。

```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}
fn main() {
    conditional_function();
}
```

