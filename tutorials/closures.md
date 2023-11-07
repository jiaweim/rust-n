# 闭包

- [闭包](#闭包)
  - [简介](#简介)
  - [捕获引用或所有权](#捕获引用或所有权)
  - [将捕获的值移出闭包](#将捕获的值移出闭包)

@author Jiawei Mao
***

## 简介

函数式编程风格包含将函数作为参数、返回值，以及将函数赋值给变量等功能。

Rust 的函数式编程功能包括：

- 闭包：可以存储在变量里的类似函数的结构
- 迭代器：一种处理元素序列的方式

**闭包**是可以保存在变量中或作为参数传递给其它函数的**匿名函数**。

与函数不同的是，闭包可以捕获调用者作用域中的值，例如：

```rust
fn main() {
   let x = 1;
   let sum = |y| x + y;

    assert_eq!(3, sum(2));
}
```

`sum` 是一个简单的闭包，它拥有输入参数 `y`，同时捕获作用域中的值 `x`。因此，调用 `sum(2)` 表示将 `2` (参数 `y`)与 `1`(`x`)相加。

## 捕获引用或所有权

闭包通过三种方式捕获环境：

- 不可变借用
- 可变借用
- 获取所有权

**示例：** 不可变引用

`only_borrows` 只是要打印值，不可变引用就够了。

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

!!! attention
    `only_borrows` 闭包的定义和调用之间，可以继续使用 `list`

**示例：** 可变引用

`borrows_mutably` 闭包需要给 `list` 增加一个元素，因此采用可变引用。

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
```

!!! attention
    `borrows_mutably` 的定义和调用之间，不能继续用 `println!` 打印 `list`。

**示例：** 所有权

闭包不严格需要所有权，如果希望强制闭包获取所有权，可以在参数列表前使用 `move` 关键字。将闭包传递给新的线程时，该技巧比较有用。

在新的线程打印 vec：

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

在该示例，尽管闭包只需要不可变引用，但是我们还是在闭包定义前添加了 `move` 关键字将 `list` 移动到闭包中。因为新线程和主线程，不确定哪个先执行完。如果主线程维护了 list 的所有权，在新线程结束之前丢弃了 list，则新线程中的不可变引用失效。因此，编译器要求将 `list` 移动到新线程中运行闭包。

## 将捕获的值移出闭包

