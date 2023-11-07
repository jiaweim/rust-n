# std::env::args

2023-11-02, 16:52
****

```rust
pub fn args() -> Args
```

返回传递给程序的命令行参数的迭代器。

第一个参数一般是可执行文件的路径，但可以设置为任意文本，因此出于安全考虑不应依赖该属性。

在 Unix 系统上，shell 对未加引号的参数展开 glob 模式（如 `*` 和 `?`）。在 Windows 上则不展开，直接原样传递参数。

## Panics

`args` 在任何参数包含无效 Unicode 字符时会 panic。如果需要接受包含无效 Unicode 字符的参数，可以使用 [args_os](args_os.md) 函数。

## 示例

```rust
use std::env;

// Prints each argument on a separate line
for argument in env::args() {
    println!("{argument}");
}
```
