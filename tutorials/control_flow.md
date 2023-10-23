# 流程控制

- [流程控制](#流程控制)
  - [简介](#简介)
  - [if 表达式](#if-表达式)
    - [else if](#else-if)
    - [if let](#if-let)
  - [循环](#循环)
    - [for 循环](#for-循环)
    - [continue](#continue)
    - [break](#break)
    - [while](#while)
    - [loop 循环](#loop-循环)
    - [从 loop 返回值](#从-loop-返回值)
    - [loop label](#loop-label)

Last updated: 2023-10-10, 13:40
@author Jiawei Mao
****

## 简介

常见控制流包括 if 表达式和循环。

## if 表达式

`if else` 表达式条件执行不同的代码分支：

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

- 测试不等于的情况

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

### else if

`else if` 实现更复杂的条件分支判断，多个条件：

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

程序会按照自上而下的顺序执行每一个分支判断，一旦成功，则跳出 if 语句块。

`else if` 太多会使代码混乱，此时可以考虑使用 `match` 语句。

### if let

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

## 循环

Rust 支持三种循环：

- loop
- while
- for

### for 循环

- for 循环语法

```rust
for 元素 in 集合 {
    // 使用元素
}
```

- 使用 for 循环遍历集合

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

!!! attention
    使用 for 时往往使用集合的引用形式，除非你后面不再使用该集合。如果不使用引用，所有权被转移到 for 语句块中，后面无法再使用该集合。

    对实现 Copy 特征的数组则不影响，`for item in arr` 不会转移所有权。

- 引用形式

```rust
for item in &container{
    // ...
}
```

- 再比如，翻转索引

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

- 修改元素

如果需要在循环中修改元素，可以使用 mut 关键字：

```rust
for item in &mut collection {
  // ...
}
```

**总结**

|用法|等价方式|所有权|
|---|---|---|
|`for item in collection`|`for item in IntoIterator::into_iter(collection)`|转移所有权|
|`for item in &collection`| `for item in collection.iter()`|不可变借用|
|`for item in &mut collection`|`for item in collection.iter_mut()`|可变借用|

- 获取元素索引

```rust
fn main() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
```

- 循环指定次数

```rust
for _ in 0..10 {
  // ...
}
```

用 `_` 替代 i 用于 for 循环，在 Rust 中 `_` 表示忽略该值。

- 两种循环方式

```rust
// 第一种
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
  let item = collection[i];
  // ...
}
// 第二种
for item in collection {

}
```

**性能**：第一种方式中 `collection[i]` 索引访问，因为边界检查导致运行时性能损坏（Rust 会检查并确认 index 在集合内），第二种方式直接迭代不会触发这种检查，编译器在编译时就完成分析并证明这种方法是合法的。

**安全**：第一种方式对 collection 的索引访问是非连续的，存在两次访问之间 collection 被修改的可能，导致脏数据产生。第二种方式是连续访问，不存在这种风险。

### continue

continue 跳过本次循环，开始下次循环：

```rust
for i in 1..4 {
    if i == 2 {
        continue;
    }
    println!("{}", i);
}
```

```
1
3
```

### break

break 直接跳出当前整个循环：

```rust
for i in 1..4 {
     if i == 2 {
         break;
     }
     println!("{}", i);
 }
```

```sh
1
```

### while

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

- 使用 loop+if+break 实现条件循环

```rust
fn main() {
    let mut n = 0;
    loop {
        if n > 5 {
            break
        }
        println!("{}", n);
        n+=1;
    }
    println!("我出来了！");
}
```

可以看出，while 循环更简洁。

### loop 循环

`loop` 是一个简单的无限循环，可以在内部通过 break 控制循环结束。

例如：

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

### 从 loop 返回值

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

### loop label

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