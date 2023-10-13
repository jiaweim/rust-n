# 字符串

## 什么是字符串

Rust 的语言核心只有一种字符串类型：字符串切片 `str`，通常以被借用的形式出现 `&str`。

## 创建字符串

`String` 是字节 `vector` 的封装，新建字符串：

```rust
let mut s = String::new();
```

## 字符串操作

### 串联字符串

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
```

相加后 `s1` 失效，