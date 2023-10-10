# Cargo

## 构建运行

- `cargo new`：创建项目
- `cargo build`：构建项目
- `cargo run`：构建并运行项目
- `cargo check`：检查是否能够编译

## 构建发布

`cargo build --release`

添加 `--release` 会优化代码，使 Rust 代码运行更快，但会延长编译时间。

## 包软件

`cargo update`

根据 Cargo.toml 文件更新 Cargo.lock 文件中的包的版本。
