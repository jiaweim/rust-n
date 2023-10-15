# 变量

- [变量](#变量)
  - [变量默认不可变](#变量默认不可变)
  - [变量绑定](#变量绑定)
  - [下划线开头的变量名](#下划线开头的变量名)
  - [变量解构](#变量解构)
    - [解构式赋值](#解构式赋值)
  - [常量](#常量)
  - [变量掩蔽](#变量掩蔽)

Last updated: 2023-10-15, 11:17
add: 变量结构
2023-10-09, 20:19
@author Jiawei Mao
****

## 变量默认不可变

Rust 中变量默认不可变（immutable），这是 Rust 实现高效并发的基础。

不能更改 immutable 变量的值。例如：

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // 报错
    println!("The value of x is: {x}");
}
```

变量前添加 `mut` 使其可变。例如：

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## 变量绑定

变量绑定，和其语言中的赋值操作相同。例如：

```rust
let a = "hello world"；
```

绑定与 Rust 的所有权的概念对应，表示将这个对象绑定给一个变量，该变量就成为它的主人。

## 下划线开头的变量名

Rust 对未使用的变量会发出警告，变量名使用下划线开头，指示 Rust 不要警告该未使用变量：

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

编译信息：

```sh
warning: unused variable: `y`                                                                                                              
 --> src\main.rs:3:9
  |
3 |     let y = 10;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default
```

可以发现，编译器只给出 `y` 未被使用的警告，忽略 `_x` 未被使用。

## 变量解构

使用 let 表达式从一个相对复杂的变量中，匹配出该变量的一部分内容，称为变量解构：

```rust
fn main() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
}
```

### 解构式赋值

Rust 1.59 之后，可以在赋值语句的左侧使用元组、切片和结构体模式。

```rust
struct Struct {
    e: i32
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

!!! note
    使用 `+=` 的赋值语句不支持解构式赋值。

## 常量

和 immutable 变量一样，常量也不能更改，但两者之间有一些区别：

- 不能对常量使用 `mut`，常量始终不可变，不能修改常量为 mut
- 常量使用 `const` 关键字声明，而不是 `let`，且必须标注类型
- 最后，常量只能设置为常量表达式，不能设置为在运行时计算的值的结果

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

常量可以在任意作用于内声明，包括全局作用于。

## 变量掩蔽

Rust 允许声明同名变量，但后面的变量会掩蔽（shadow）掉前面的变量。例如：

```rust
fn main() {
    let x = 5;

    // 在 main 函数的作用域内掩蔽上面的 x
    let x = x + 1;
    {
        // 在当前 {} 作用域内，掩蔽之前的 x
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

```
The value of x in the inner scope is: 12
The value of x is: 6
```

掩蔽与 mut 变量不同，使用 `let` **掩蔽生成了完全不同的新变量**，只是恰好与上一个变量名称相同。掩蔽涉及内存对象的再分配，而 `mut` 变量只能修改同一个内存地址上的值，不涉及内存对象的再分配，性能更好。

掩蔽的唯一作用，大概是不用想新的名称，直接使用不再使用的变量的名称：

```rust
// 字符串类型
let spaces = "   ";
// usize数值类型
let spaces = spaces.len();
```


