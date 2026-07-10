# 文档系统

模板使用 [mdbook](https://rust-lang.github.io/mdBook/) 管理项目文档，所有源文件位于 `docs/src/`。

## 文件说明

```
docs/
├── book.toml          # mdbook 配置文件
└── src/
    ├── SUMMARY.md      # 目录结构
    ├── 01-overview.md  # 模板概述
    ├── 02-config.md    # 项目配置说明
    ├── 03-source.md    # 源代码结构
    ├── 04-docs.md      # 文档系统（本页）
    └── 05-cicd.md      # CI / CD
```

---

## `book.toml` 关键配置

```toml
[book]
title = "rust-project 设计文档"
language = "zh-CN"

[build]
build-dir = "book"              # 构建产物输出到 docs/book

[output.html]
git-repository-url = "https://github.com/TODO/rust-project"
edit-url-template = "https://github.com/TODO/rust-project/edit/main/docs/src/{path}"
site-url = "/rust-project/"     # GitHub Pages 子路径
```

| 配置项 | 说明 |
|--------|------|
| `build-dir` | 设为 `"book"`，GitHub Actions 会从 `docs/book` 部署 |
| `site-url` | 必须设为 `/<仓库名>/`，否则 GitHub Pages 上 CSS/JS 会 404 |
| `edit-url-template` | 文档页面的"编辑"链接指向 GitHub |
| `git-repository-url` | 页面右上角的 GitHub 图标链接 |

拿到模板后，需要把 `title`、`git-repository-url`、`edit-url-template`、`site-url` 中的 `TODO` / `rust-project` 替换为实际值。

---

## 本地预览

```bash
cargo install mdbook
cd docs && mdbook serve --open
```

`mdbook serve` 会在 `http://localhost:3000` 启动一个支持热更新的本地服务器。

---

## 构建

```bash
mdbook build docs
```

产物输出到 `docs/book/`，可直接用任意静态文件服务托管。
