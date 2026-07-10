# rust-project

> Rust 项目模板，拿到后按以下清单逐项修改即可快速启动。

---

## 一、必改清单

### 1. 项目元信息 — `Cargo.toml`

- [ ] `name` — 改为实际 crate 名称
- [ ] `description` — 填写项目简介
- [ ] `authors` — 填写作者信息
- [ ] `repository` — 改为实际 GitHub 仓库地址
- [ ] `dependencies` — 按需添加依赖

### 2. 文档配置 — `docs/book.toml`

- [ ] `title` — 改为"xxx 设计文档"
- [ ] `authors` — 填写作者
- [ ] `git-repository-url` — 改为实际仓库地址
- [ ] `edit-url-template` — 同上
- [ ] `site-url` — 改为 `/<你的仓库名>/`（GitHub Pages 路径前缀）

### 3. Rust 工具链 — `rust-toolchain.toml`

- [ ] `channel` — 按需改为 `nightly` 或指定具体版本如 `"1.80.0"`

### 4. 代码风格 — `rustfmt.toml`

- [ ] 按团队习惯调整 `max_width`、`tab_spaces` 等参数

### 5. CI 部署 — `.github/workflows/deploy-mdbook.yml`

- [ ] `branches` — 确认目标分支是 `main` 还是其他分支

### 6. 启用 GitHub Pages

一次设置即可：

1. 仓库 **Settings → Pages**
2. **Source** 选择 **GitHub Actions**
3. 保存 — 后续每次 push 自动部署

---

## 二、模板包含的文件

| 文件 | 用途 |
|------|------|
| `Cargo.toml` | 项目清单，含 release 优化配置 |
| `rust-toolchain.toml` | 锁定 stable 工具链 |
| `rustfmt.toml` | 统一代码格式化规则 |
| `.gitattributes` | 规范换行符与 diff 行为 |
| `.gitignore` | 忽略 target、Cargo.lock 等 |
| `Makefile` | 常用命令快捷入口 |
| `src/main.rs` | 可执行入口 |
| `src/lib.rs` | 库入口 |
| `examples/` | 可运行示例程序 |
| `tests/` | 集成测试 |
| `docs/` | mdbook 文档站点，含 5 个说明章节 |
| `.github/workflows/deploy-mdbook.yml` | 自动部署文档到 GitHub Pages |

详细说明见各章节文档，本地预览：

```bash
cargo install mdbook
cd docs && mdbook serve --open
```

---

## 三、代码起手

```bash
# 或直接用 Makefile
make build    # 开发构建
make run      # 运行
make test     # 测试
make fmt      # 格式化
make clippy   # lint 检查
```

---

## 四、可选后续

- [ ] 添加 `Dockerfile`
- [ ] 添加 CI 测试流程（`cargo test`、`cargo clippy`）
- [ ] 添加 `CHANGELOG.md`
- [ ] 添加 `LICENSE` 文件
- [ ] 配置 `clippy.toml`
