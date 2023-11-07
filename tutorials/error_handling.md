# 错误处理

- [错误处理](#错误处理)
  - [简介](#简介)
  - [panic! 处理不可恢复错误](#panic-处理不可恢复错误)
    - [终止 panic](#终止-panic)
    - [何时用 panic!](#何时用-panic)
  - [Result 处理可恢复错误](#result-处理可恢复错误)
    - [枚举 Result](#枚举-result)
    - [处理错误](#处理错误)
    - [unwrap 和 expect](#unwrap-和-expect)
    - [错误传播](#错误传播)
    - [用 ? 简化](#用--简化)

Last updated: 2023-10-14, 21:36
@author Jiawei Mao
****

## 简介

Rust 将错误分为两类：

- 可恢复的（recoverable）
- 不可恢复的（unrecoverable）

对可恢复错误，如未找到文件，可能只需向用户报告问题并提示重试。不可恢复错误则是出 bug 的征兆，比如越界访问数组，此时要立即停止程序。

大多数语言不区分这两种错误，并采用类似异常的方式统一处理。Rust 则予以区分：

- 用 `Result<T, E>` 类型处理可恢复错误
- 用 `panic!` 宏处理不可恢复错误

## panic! 处理不可恢复错误

`panic!` 用于处理不可恢复错误。触发 panic 的方式有两种：

- 被动：执行会造成 panic 的代码（比如越界访问数组）
- 主动：显式调用 `panic!` 宏

`panic` 通常会打印一个错误消息，展开并清理栈数据，然后退出。

**示例：** 被动触发

数组越界访问，触发 panic。

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

**示例：** 主动调用

使用 `panic!` 宏主动触发 panic。

```rust
fn main() {
    panic!("crash and burn");
}
```

### 终止 panic

Rust 提供了两种处理 panic 的方式：**栈展开**和**直接终止**。默认为栈展开。

- **栈展开**回溯栈上数据和函数调用，有更多的善后工作要做，但给出充分的报错信息和栈调用信息
- **直接终止**不清理数据直接退出程序，善后工作交给操作系统

大多时候，使用默认最好。

如果希望减少最终编译出的二进制文件大小，可以使用直接终止的方式。需要配置 `Cargo.toml` 文件：

```toml
[profile.release]
panic = 'abort'
```

!!! attention
    线程 panic 后，如果是 main 线程，则程序会终止；如果是其它子线程，则该线程终止，但不影响 main 线程。

### 何时用 panic!

`Result<T, E>` 枚举类型表示函数的返回结果：

- 没有错误时，返回用 `Result` 类型包裹的 `Ok(T)`
- 发生错误时，返回 `Err(E)`

`Result<T, E>` 定义：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

对 `Result` 返回值最直接的处理方式是 `unwrap` 和 `expect`:

- `unwrap` 成功则返回值，失败就 panic
- `expect` 功能与 `unwrap` 一样，只是可以自定义错误信息

**示例：** unwrap

```rust
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

`parse` 试图将字符串 "127.0.0.1" 解析为一个 IP 地址类型 `IpAddr`，返回 `Result<IpAddr, E>` 类型，后面的 `unwrap()`：

- 如果解析成功，则把 `Ok(IpAddr)` 中的值赋给 `home`
- 如果失败，直接 panic

应用 `panic!` 的场景：

- 示例、原型、测试等需要快速开发的场景，此时跳过错误处理，回头有需要再处理
- 确定代码正确
- 内存安全问题，如数组越界

## Result 处理可恢复错误

`Result<T, E>` 是一种更温和的错误处理方式，其定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 泛型参数 `T` 表示成功时存入的正确值类型，存入 `Ok(T)`
- 泛型参数 `E` 代表错误时存入的错误值，存放 `Err(E)`

以打开文件为例：

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
}
```

`File::open` 返回 `Result<std::fs::File, std::io::Error>` 类型，说明 `File::open` 调用成功时返回可以进行读写的文件句柄；调用失败则返回一个 IO 错误。

### 枚举 Result

通过 `Result` 获取返回信息：

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

代码很清晰，打开文件后对 `Result<T, E>` 进行匹配取值：

- 如果成功，则将 `Ok(file)` 中存储的文件句柄 `file` 赋值给 `f`
- 如果失败，则将 `Err(error)` 中存放的错误信息 `error` 使用 `panic!` 抛出来

### 处理错误

碰到错误直接 `panic!` 过于粗暴，有些错误时可以处理的：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

在匹配到 error 后，继续对 error 进行匹配解析：

- 如果是文件不存在 `ErrorKind::NotFound`，就创建文件，`File::create` 创建文件也返回 Result，继续用 match 进行处理，创建成功，就新建的文件句柄赋值给 `f`；失败则 panic
- 剩下的错误，全部 panic

虽然很清晰，但是代码有些啰嗦。

### unwrap 和 expect

`match` 匹配分支太麻烦，`unwrap` 和 `expect` 快速处理 Result：

- 如果成功，将 `Ok(T)` 中的值取出来
- 如果失败，直接 panic

**示例：** unwrap

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

如果 `hello.txt` 文件不存在，`unwrap` 直接 `panic`。

**expect** 跟 unwrap 很像，也是遇到错误直接 panic，但会带上自定义的错误提示信息：

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

### 错误传播

从文件中读取用户名的函数：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}
```

说明：

- 该函数返回一个 `Result<String, io::Error>` 类型，读取用户名成功时，返回 `Ok(String)`，失败时 `Err(io::Error)`
- `File::open` 和 `f.read_to_string` 返回的 `Result<T, E>` 中的 `E` 就是 `io::Error`

该函数将 `io::Error` 的错误往上传播，最终由函数的调用者进行处理。

但是，上面的代码**太长了**。

### 用 ? 简化

`?` 是一个宏，功能与 `match` 几乎完全相同。

```rust
use std::fs::File;
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`？` 功能：

- 如果结果是 `Ok(T)`，则将 `T` 赋值给 `f`
- 如果结果是 `Err(E)`，则返回该错误，所以 `?` 特别适合用来传播错误


链式调用可以进一步就简化代码：

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

