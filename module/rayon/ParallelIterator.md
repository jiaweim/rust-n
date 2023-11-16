# rayon::iter::ParallelIterator

## flat_map

```rust
fn flat_map<F, PI>(self, map_op: F) -> FlatMap<Self, F>
where
    F: Fn(Self::Item) -> PI + Sync + Send,
    PI: IntoParallelIterator,
```

将 `map_op` 应用于此迭代器的每个元素获得嵌套并行迭代器，并将嵌套的迭代器展开为单个并行迭代器。

```rust
use rayon::prelude::*;

let a = [[1, 2], [3, 4], [5, 6], [7, 8]];

let par_iter = a.par_iter().cloned().flat_map(|a| a.to_vec());

let vec: Vec<_> = par_iter.collect();

assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6, 7, 8]);
```

## flat_map_iter

```rust
fn flat_map_iter<F, SI>(self, map_op: F) -> FlatMapIter<Self, F>
where
    F: Fn(Self::Item) -> SI + Sync + Send,
    SI: IntoIterator,
    SI::Item: Send, 
```

将 `map_op` 应用于当前迭代器的每个元素生成嵌套的串行迭代器，然后将嵌套迭代器展开获得新的并行迭代器。

**flat_map_iter vs. flat_map**

两个方法功能相似，但行为略有不同：

- `flat_map` 的嵌套迭代器也必须是并行的，它们会通过嵌套并行进一步拆分
- `flat_map_iter` 的嵌套迭代器是串行 `Iterator`


```rust
use rayon::prelude::*;
use std::cell::RefCell;

let a = [[1, 2], [3, 4], [5, 6], [7, 8]];

let par_iter = a.par_iter().flat_map_iter(|a| {
    // The serial iterator doesn't have to be thread-safe, just its items.
    let cell_iter = RefCell::new(a.iter().cloned());
    std::iter::from_fn(move || cell_iter.borrow_mut().next())
});

let vec: Vec<_> = par_iter.collect();

assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6, 7, 8]);
```
