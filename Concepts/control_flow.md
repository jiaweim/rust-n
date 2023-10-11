# 控制流

- [控制流](#控制流)
  - [1. 简介](#1-简介)
  - [2. if 表达式](#2-if-表达式)
    - [2.1. else if](#21-else-if)
    - [2.2. if let](#22-if-let)
  - [3. 循环](#3-循环)
    - [3.1. loop](#31-loop)
    - [3.2. 从 loop 返回值](#32-从-loop-返回值)
    - [3.3. loop label](#33-loop-label)
    - [3.4. while](#34-while)
    - [3.5. for](#35-for)

Last updated: 2023-10-10, 13:40
@author Jiawei Mao
****

## 1. 简介

常见控制流包括 if 表达式和循环。

## 2. if 表达式

示例：

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

`if` 后面为条件语句，条件语句值必须为 `bool` 类型，Rust 不会自动将非 `bool` 类型转换为 `bool` 值。

测试不等于的情况：

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

### 2.1. else if

多个条件：

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

`else if` 太多会使代码混乱，此时可以考虑使用 `match` 语句。

### 2.2. if let

if 是 expression，会返回值，因此可以用在 let 语句中赋值。

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

对这种赋值 if 语句，要求每个分支返回的类型相同，否则报错。例如：

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

这里 if 部分值为整数，else 部分值为字符串。Rust 是静态类型语言，在编译时必须知道变量类型，因此这样行不通。

## 3. 循环

Rust 支持三种循环：

- loop
- while
- for

### 3.1. loop

`loop` 用于执行循环。例如：

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

运行该程序，会不停打印 `again!`，直到手动停止。大多数终端支持用 `ctrl-c` 中断程序。

- 也可以在 `loop` 内部使用 `break` 关键字终止循环。
- 使用 `continue` 跳过这次循环余下的代码，进入写一次循环

### 3.2. 从 loop 返回值

`loop` 的一个重要功能是尝试可能失败的操作，例如检查线程是否完成其工作。在操作完成后，还可能需要将操作结果传递到循环外部。为此，可以在 break 表达式后面添加要返回的值。例如：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

在循环前声明了 `counter` 变量并初始化为 0。然后声明变量 `result` 保存 `loop` 的返回值。在循环的每次迭代中，给 counter 变量 +1，然后检查 counter 是否等于 10，如果是，就用 break 中断循环，并 返回 counter * 2。最后 result 值为 20。

### 3.3. loop label

对嵌套循环，break 和 continue 只作用于当前循环，添加标签后，则可以灵活地作用于外层循环。循环标签必须以**单引号**开头。例如：

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

这里外层循环包含一个 `'counting_up` 标签。内部循环有 2 个 break 语句，第一个 break 没有指定标签，只能中断内部循环，第二个 `break 'counting_up;` 则中断外部循环。运行输出：

```
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

### 3.4. while

**示例：** 使用 while 循环三次

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### 3.5. for

可以用 while 循环遍历集合，例如：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

但这种方式容易出错。使用 for 循环更方便：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

再比如，翻转索引：

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
