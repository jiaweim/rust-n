## binary_search_by

```rust
pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
where
    F: FnMut(&'a T) -> Ordering,
```

