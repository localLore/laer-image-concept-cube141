# 模板概述

本仓库是一个 Rust 项目模板，提供了一套开箱即用的工程化基础设施，拿到后只需少量修改即可启动开发。

## 模板包含

| 类别 | 内容 |
|------|------|
| Rust 工具链 | `rust-toolchain.toml` 锁定 stable 通道 |
| 代码风格 | `rustfmt.toml` 统一格式化规则 |
| Git 配置 | `.gitattributes` 规范换行符与 diff 行为，`.gitignore` 忽略构建产物 |
| 项目清单 | `Cargo.toml` 含 release 优化配置 |
| 源码骨架 | `src/main.rs`（可执行入口）+ `src/lib.rs`（库入口） |
| 文档站点 | `docs/` 基于 mdbook 的文档系统，含 5 个章节 |
| Makefile | 常用命令快捷入口（`make build`、`make test` 等） |
| 示例 | `examples/` 可运行示例程序 |
| 测试 | `tests/` 集成测试目录 |
| CI/CD | `.github/workflows/deploy-mdbook.yml` 自动部署文档到 GitHub Pages |

## 项目文件树

```
.
├── .gitattributes
├── .gitignore
├── .github/
│   └── workflows/
│       └── deploy-mdbook.yml
├── Cargo.toml
├── Makefile
├── README.md
├── docs/
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       ├── 01-overview.md
│       ├── 02-config.md
│       ├── 03-source.md
│       ├── 04-docs.md
│       └── 05-cicd.md
├── examples/
│   └── demo.rs
├── rust-toolchain.toml
├── rustfmt.toml
├── src/
│   ├── main.rs
│   └── lib.rs
└── tests/
    └── integration_test.rs
```
