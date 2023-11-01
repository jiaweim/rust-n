# std::mem::take

2023-11-01, 13:50
****

```rust
pub fn take<T>(dest: &mut T) -> T
where
    T: Default,
```

使用 `T` 的默认值替换 `dest`，返回 `dest` 原来的值：

- 交换两个变量的值，用 [swap](swap.md)
- 用指定值而非默认值，用 [replace](replace.md)

## Examples

简单示例：

```rust
use std::mem;

let mut v: Vec<i32> = vec![1, 2];

let old_v = mem::take(&mut v);
assert_eq!(vec![1, 2], old_v);
assert!(v.is_empty());
```

`take` 将将 struct 字段替换为默认值，并获取字段所有权。

没有 `take`，不好处理下面的问题：

```rust
struct Buffer<T> { buf: Vec<T> }

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // error: cannot move out of dereference of `&mut`-pointer
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}
```

注意，`T` 不一定实现 `Clone`，因为也不能克隆并重置 `self.buf`。但是 `take` 可以从 `self` 分离 `self.buf` 的原始值：

```rust
use std::mem;

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::take(&mut self.buf)
    }
}

let mut buffer = Buffer { buf: vec![0, 1] };
assert_eq!(buffer.buf.len(), 2);

assert_eq!(buffer.get_and_reset(), vec![0, 1]);
assert_eq!(buffer.buf.len(), 0);
```


## 参考

- https://doc.rust-lang.org/std/mem/fn.take.html