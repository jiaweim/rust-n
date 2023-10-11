# struct

## 简介

结构体（struct）是一种自定义数据类型，可以将多个相关值打包在一起。struct 类似面向对象语言中的对象属性。

## 定义 struct

struct 类似元组，都保存多个相关的值。和元组一样，struct 的各个部分可以是不同类型；与元祖不同的时，struct 中每条数据具有名称。

示例：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

使用：

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

在实例化 struct 时，顺序不重要。

struct 字段通过点号 `.` 访问，例如 `user1.email`。