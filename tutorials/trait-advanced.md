# 特征进阶

- [特征进阶](#特征进阶)
  - [关联类型](#关联类型)
  - [默认泛型参数](#默认泛型参数)
  - [调用同名方法](#调用同名方法)
    - [优先调用类型上的方法](#优先调用类型上的方法)
    - [调用特征的方法](#调用特征的方法)
  - [完全限定语法](#完全限定语法)
  - [特征定义中的特征约束](#特征定义中的特征约束)
  - [在外部类型上实现外部特征](#在外部类型上实现外部特征)

2023-10-26, 18:58
@author Jiawei Mao
****

## 关联类型

关联类型是在特征定义的语句块中声明一个自定义类型，这样就可以在特征的方法签名中使用该类型：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

这是标准库中的迭代器特征 `Iterator`，它有一个 `Item` 关联类型，用于替代遍历的值的类型。

同时， `next` 方法也返回 `Item` 类型，不过使用 `Option` 枚举进行了包裹，假如迭代器中的值是 `i32` 类型，那么调用 `next` 方法将获取一个 `Option<i32>` 的值。

`Self` 用来指代当前调用者的具体类型，那么 `Self::Item` 就用来指代该类型实现中定义的 `Item` 类型：

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}

fn main() {
    let c = Counter{..}
    c.next()
}
```

这里为 `Counter` 类型实现 `Iterator` 特征，变量 `c` 是特征 `Iterator` 的实例，也是 `next` 方法的调用者。对 `next` 方法而言， `Self` 是调用者 `c` 的具体类型 `Counter`，而 `Self::Item` 是 `Counter` 中定义的 `Item` 类型 `u32`。

为何不用泛型：

```rust
pub trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}
```

答案很简单，为了代码的可读性，使用泛型后，需要在所有地方都写 `Iterator<Item>`，而使用关联类型，只需要写 `Iterator`，当类型定义复杂时，这种写法可以增加可读性：

```rust
pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
  type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
  fn is_null(&self) -> bool;
}
```

这里 `Address` 的写法比 `AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash` 要简单的多，而且含义清晰。

再例如，如果使用泛型，你将得到以下的代码：

```rust
trait Container<A,B> {
    fn contains(&self,a: A,b: B) -> bool;
}

fn difference<A,B,C>(container: &C) -> i32
  where
    C : Container<A,B> {...}
```

可以看到，由于使用了泛型，导致函数头部也必须增加泛型的声明，而使用关联类型，将得到可读性好得多的代码：

```rust
trait Container{
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference<C: Container>(container: &C) {}
```

## 默认泛型参数

使用泛型类型参数时可以指定一个默认的具体类型，例如标准库中的 `std::ops::Add` 特征：

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

它有一个泛型参数 `RHS`，指定默认为 `Self`。也就是当用户不指定 `RHS` 时，默认使用两个同样类型的值进行相加，然后返回一个关联类型 `Output` 。

例如：

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

上面为 `Point` 结构体提供 `+` 运算符重载，不过 Rust 不支持创建自定义运算符，也无法重载所有运算符，目前来只能重载定义在 `std::ops` 中的运算符。

跟 `+` 对应的特征是 `std::ops::Add`，前面有提到它的定义 `trait Add<RHS=Self>`，但是上例没有为 `Point` 实现 `Add<RHS>` 特征，而是实现了 `Add` 特征（没有类型参数），这意味着使用了 `RHS` 的默认类型 `Self`。换句话说，这里定义的是两个相同 `Point` 类型相加，因此无需指定 `RHS`。

- 下面创建两个不同类型的相加

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

这里实现 `Millimeters + Meters` 的 `+` 操作，因此不能再使用默认的 `RHS`，否则就变成 `Millimeters + Millimeters` 形式。使用 `Add<Meters>` 可以将 `RHS` 指定为 `Meters`，`fn add(self, rhs: RHS)` 就变成 `Millimeters` 和 `Meters` 相加。

默认类型参数有两个作用：

1. 减少样板代码
2. 扩展类型但是无需大幅修改现有的代码

## 调用同名方法

不同特征拥有同名方法很正常：

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

这里，不仅仅两个特征 `Pilot` 和 `Wizard` 有 `fly` 方法，就连实现那两个特征的 `Human` 结构体也拥有一个同名方法 `fly`。

下面讲讲该如何调用这些 fly 方法。

### 优先调用类型上的方法

当调用 `Human` 实例的 `fly` 时，编译器默认调用该类型中定义的方法：

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

```sh
*waving arms furiously*
```

说明直接调用了类型上定义的方法。

### 调用特征的方法

特征的方法需要显式调用：

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法
}
```

```sh
This is your captain speaking.
Up!
*waving arms furiously*
```

因为 `fly` 方法的参数是 `self`，显式调用时编译器可以根据调用的类型( `self` 的类型)决定具体调用哪个方法。

这个时候问题又来了，如果方法没有 `self` 参数呢？如关联函数。

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

就像人类妈妈会给自己的宝宝起爱称一样，狗妈妈也会。狗妈妈称呼自己的宝宝为 **Spot**，其它动物称呼狗宝宝为 **puppy**，这个时候假如有动物不知道该如何称呼狗宝宝，它需要查询一下。

`Dog::baby_name()` 的调用方式显然不行，这是狗妈妈对宝宝的爱称，可能你会想到通过下面的方式查询其他动物对狗狗的称呼：

```rust
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

报错了：

```sh
error[E0283]: type annotations needed // 需要类型注释
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^ cannot infer type // 无法推断类型
   |
   = note: cannot satisfy `_: Animal`
```

因为单纯从 `Animal::baby_name()` 编译器无法得到有效的信息：实现 `Animal` 特征的类型可能有很多，你究竟是想获取哪个动物宝宝的名称？狗宝宝？猪宝宝？还是熊宝宝？此时，就需要使用完全限定语法。

## 完全限定语法

完全限定语法是调用函数最为明确的方式：

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

在尖括号中通过 `as` 关键字提供类型注解，即 `Animal` 就是 `Dog` ，而不是其他动物，因此最终会调用 `impl Animal for Dog` 中的方法，获取到其它动物对狗宝宝的称呼：`puppy`。

**完全限定语法**定义为：

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

上面定义中，第一个参数是方法接收器 `receiver` （三种 `self` ），只有方法才拥有，关联函数
就没有 receiver 。

完全限定语法可用于任何函数或方法调用，大多时候 Rust 编译器能根据上下文自动推导出调用路径，因此无需使用完全限定语法。只有当存在多个同名函数或方法，Rust 无法区分出你想调用的目标函数时，才必须使用该语法。

## 特征定义中的特征约束

有时会需要让某个特征 `A` 能使用另一个特征 `B` 的功能，因此要为类型实现特征 A 和 B。

例如有一个特征 `OutlinePrint` ，它有一个方法，能够对当前的实现类型进行格式化输出：

```rust
use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

`OutlinePrint: Display` 表示如果要实现 `OutlinePrint`，首先要实现 `Display`。

## 在外部类型上实现外部特征

前面有提到，特征或类型必须至少有一个是本地的，才能在此类型上定义特征。

但是通过 newtype 可以绕过该规则;