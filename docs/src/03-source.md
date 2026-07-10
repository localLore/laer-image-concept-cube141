# 源代码结构

## 目录

```
src/
├── main.rs    # 可执行入口
└── lib.rs     # 库入口
```

---

## `src/main.rs`

最小的可执行入口，仅输出一行问候，用于验证项目能正常编译运行：

```rust
fn main() {
    println!("Hello, world!");
}
```

如果项目是纯库（无二进制目标），删除此文件并移除 `Cargo.toml` 中对应的 `[[bin]]`（如无显式声明则无需操作）。

---

## `src/lib.rs`

库入口，内含 crate 级文档注释，用于承载项目整体说明：

```rust
//! # rust-project
//!
//! TODO: 填写 crate 级文档，说明本库的用途与设计理念。
```

`//!` 开头的注释为 crate 级文档，会被 `cargo doc` 渲染到文档首页。拿到模板后应替换为项目自身的描述。

---

## 扩展模块

当项目规模增长时，可在 `src/` 下按功能拆分子模块：

```
src/
├── main.rs
├── lib.rs
├── config.rs    // 配置解析
├── service.rs   // 业务逻辑
└── util.rs      // 工具函数
```

在 `lib.rs` 或 `main.rs` 中使用 `mod` 声明即可引入：

```rust
mod config;
mod service;
mod util;
```

---

## `Makefile`

常用命令的快捷入口，无需记忆 cargo 参数：

| 命令 | 等价操作 |
|------|----------|
| `make build` | `cargo build` |
| `make test` | `cargo test` |
| `make run` | `cargo run` |
| `make fmt` | `cargo fmt` |
| `make clippy` | `cargo clippy -- -D warnings` |
| `make docs` | `mdbook build docs` |
| `make docs-serve` | `mdbook serve docs --open` |
| `make example` | 运行所有示例（`EXAMPLES` 变量控制） |
| `make example_demo` | 运行指定示例 `cargo run --example demo` |
| `make clean` | `cargo clean` + 删除 `docs/book` |

`EXAMPLES` 变量定义了示例列表，新增示例只需往列表里加名字：

```makefile
EXAMPLES = demo foo bar
```

---

## `examples/`

存放可运行的示例程序。每个 `.rs` 文件对应一个独立可执行示例：

```bash
cargo run --example demo
```

模板中提供了一个最小示例 `demo.rs`，可根据项目需要扩展。

---

## `tests/`

存放集成测试。`cargo test` 会自动发现并运行此目录下的所有 `.rs` 文件，每个文件作为独立 crate 编译并链接到主库。

```bash
cargo test
```

与 `src/` 中的 `#[cfg(test)] mod tests { ... }`（单元测试）不同，`tests/` 中的测试从外部视角验证公开 API。
