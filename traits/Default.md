# Default

2023-11-16, 15:57
****

许多类型都有默认值：向量和字符串默认为空，数值默认为 0，`Option` 默认为 `None` 等。

提供默认值的类型实现 `std::default::Default` trait：

```rust
trait Default {
    fn default() -> Self;
}
```

`default` 方法返回一个 `Self` 类型的值。`String` 的 `Default` 实现如下：

```rust
impl Default for String {
    fn default() -> String {
        String::new()
    }
}
```

Rust 的所有集合类型，如 `Vec`, `HashMap`, `BinaryHeap` 都实现了 `Default`，其 `default` 方法返回一个空集合。当需要构建一些值的集合，但又想让调用者决定构建何种集合时，这很有用。例如，`Iterator` trait 的 `partition` 将迭代器生成的元素分为两个元素，并使用闭包决定值的去向。

```rust
use std::collections::HashSet;
let squares = [4, 9, 16, 25, 36, 49, 64];
let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
    = squares.iter().partition(|&n| n & (n - 1) == 0);

assert_eq!(powers_of_two.len(), 3);
assert_eq!(impure.len(), 4);
```

闭包 `|&n| n & (n - 1) == 0` 使用位操作识别哪些数值是 2 的幂，`partition` 使用它生成两个 `HashSet`。`partition` 也可以生成其它集合，只要该集合实现 `Default` 提供初始空集合，实现 `Extend<T>` 以将 T 添加到集合中即可。`String` 实现了 `Default` 和 `Extend<char>`，所以可以这样写：

```rust
let (upper, lower): (String, String)
    = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
assert_eq!(upper, "GTO");
assert_eq!(lower, "reat eacher nizuka");
```

`Default` 还常用于为包含大量参数集合的结构体生成默认值。如 `glium` crate 为强大而复杂的 OpenGL 图形库提供了 Rust 绑定。`glium::DrawParameters` 结构体包含 24 个字段，每个字段控制 OpenGL 应该如何渲染某些图形的不同细节。glium `draw` 函数以 `DrawParameters` 结构体为参数。由于 `DrawParameters` 已经实现了 `Default`，因此只需设置想要更改的字段即可创建一个 `DrawParameters` 传给 `draw` 的结构体：

```rust
let params = glium::DrawParameters {
    line_width: Some(0.02),
    point_size: Some(0.02),
    .. Default::default()
};

target.draw(..., &params).unwrap();
```

这里会调用 `Default::default()` 来创建 `DrawParameters`，该值会使用所有字段的默认值进行初始化，然后使用结构体的 `..` 语法创建一个更改了 `line_width` 和 `point_size` 的新值。

如果类型 T 实现了 Default，那么标准库会自动为 `Rc<T>`, `Arc<T>`, `Box<T>`, `Cell<T>`, `RefCell<T>`, `Cow<T>`, `Mutex<T>`, `RwLock<T>` 实现 `Default`。例如，类型 `Rc<T>` 的默认值是一个指向类型 `T` 的默认值的 `Rc`。

如果一个元组类型的所有元素类型都实现了 Default，那么该元组也同样会实现 `Default`，这个元组的默认值包含每个元素的默认值。

Rust 不会为结构体类型隐式实现 `Default`，但是如果结构体的所有字段都实现了 `Default`，则可以使用 `#[derive(Default)]` 为此结构体自动实现 `Default`。
