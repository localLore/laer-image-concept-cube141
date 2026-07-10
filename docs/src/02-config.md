# 项目配置

## `Cargo.toml`

Rust 项目的核心清单文件，模板中预设了以下内容：

```toml
[package]
name = "rust-project"
version = "0.1.0"
edition = "2021"
description = "TODO: 填写项目描述"
authors = ["TODO: 填写作者"]
license = "MIT"
repository = "https://github.com/TODO/rust-project"

[dependencies]

[dev-dependencies]

[profile.release]
opt-level = 3      # 最高优化级别
lto = true         # 链接时优化
codegen-units = 1  # 单代码生成单元，更小更快的二进制
```

拿到模板后需要把 `name`、`description`、`authors`、`repository` 改为实际值，并按需添加 `dependencies`。

---

## `rust-toolchain.toml`

锁定 Rust 工具链版本，确保所有开发者和 CI 使用一致的编译器：

```toml
[toolchain]
channel = "stable"
```

项目会强制使用 `stable` 通道。如需 nightly 特性，改为 `channel = "nightly"`。

---

## `rustfmt.toml`

统一代码格式化规则：

```toml
max_width = 100
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Max"
```

| 配置项 | 说明 |
|--------|------|
| `max_width` | 单行最大宽度 100 字符 |
| `tab_spaces` | 缩进 4 空格 |
| `edition` | 使用 2021 edition 的格式化规则 |
| `newline_style` | 统一 Unix 换行符 (LF) |
| `use_small_heuristics` | 使用较紧凑的排版启发式 |

运行 `cargo fmt` 即可按此规则自动格式化全部代码。

---

## `.gitattributes`

规范 Git 对各类文件的行为：

```
*.rs    text diff=rust     # Rust 源码按文本处理，使用 Rust 专用 diff 驱动
Cargo.lock  binary         # 锁定文件按二进制处理，避免合并时产生无意义冲突
*.toml  text eol=lf        # 配置文件统一 LF 换行
*.yaml  text eol=lf
*.yml   text eol=lf
*.json  text eol=lf
```

---

## `.gitignore`

忽略不需要纳入版本控制的文件：

```
/target          # 构建产物
**/*.rs.bk       # 格式化备份
Cargo.lock       # 库项目建议不提交锁文件（二进制项目可按需删除此行）
```
