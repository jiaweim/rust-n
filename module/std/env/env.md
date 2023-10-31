# std::env

## 简介

该模块用于检查和操作进程的环境。

该模块包含检查环境信息的各种函数，例如环境变量，进程参数，当前目录以及各种其它重要目录。

该模块中有几个函数和结构都有对应的以 `os` 结尾的同名对象。 以 `os` 结尾的那些将返回 `OsString`，而没有以 `os` 结尾的那些将返回 `String`。


## 函数

### args

```rust
pub fn args() -> Args
```

返回程序启动的参数（通常通过命令行传递）。

第一个参数一般是可执行文件的路径，但它可以设置为任意文本，因此出于安全目的不应依赖该属性。

在 Unix 系统上，shell 常使用 glob 模式（如 `*` 和 `?`）展开未加引号的参数。在 Windows 上则不会展开，直接原样传递参数。

**Panics**

**示例**

```rust
use std::env;

// Prints each argument on a separate line
for argument in env::args() {
    println!("{argument}");
}
```

