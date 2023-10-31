# core::ops::Range

## 简介

```rust
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}
```

定义区间 `[start, end)`。

如果 `start >= end`，则为空。

## 示例

`start..end` 语法生成的就是 `Range`：

```rust
assert_eq!((3..5), std::ops::Range { start: 3, end: 5 });
assert_eq!(3 + 4 + 5, (3..6).sum());
```

```rust
let arr = [0, 1, 2, 3, 4];
assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
assert_eq!(arr[ .. 3], [0, 1, 2      ]);
assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
assert_eq!(arr[1.. 3], [   1, 2      ]); // This is a `Range`
assert_eq!(arr[1..=3], [   1, 2, 3   ]);
```

## 参考

- https://doc.rust-lang.org/core/ops/struct.Range.html