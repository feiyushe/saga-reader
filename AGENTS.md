# AGENTS.md — Saga Reader 项目工作指南

> 本文件定义 AI Agent 在本项目中的工作规范。项目知识库见 `.ai-context/`。

---

## 1. 会话启动流程

每次会话开始时，按顺序阅读：

1. **`.ai-context/SKILL.md`** — 获取项目全貌与关键路径
2. **`.ai-context/references/PROJECT-ESSENCE.md`** — 理解项目定位与核心约束
3. 根据任务类型选读：
   - 跨组件开发 → `ARCHITECTURE.md`
   - 修改设计模式 → `DECISIONS.md`
   - 排查问题 → `DYNAMICS.md`

---

## 2. 项目结构与工作方式

### 技术栈

| 层 | 技术 | 说明 |
|----|------|------|
| 桌面框架 | Tauri 2.x | Rust 后端 + Web 前端 |
| 前端 | SvelteKit 2.x + Svelte 5 | 静态适配器，TailwindCSS 样式 |
| UI 组件库 | Skeleton 3.x | 见 `app/src/lib/widgets/` |
| 后端 | Rust (edition 2024) | Cargo workspace |
| 数据库 | SQLite + SeaORM | 本地存储，见 `crates/recorder/` |
| 包管理 | Bun（推荐）/ npm / pnpm | Bun 镜像配置见 `bunfig.toml` |

### 常用命令

```bash
bun install          # 安装依赖
bun run dev          # 启动开发（Tauri dev 模式）
bun run build        # 构建桌面应用
bun run build:macos  # 构建 macOS Universal
```

### 代码规范

- **Rust**: 遵循 `clippy.toml` 配置；使用 `anyhow` 做错误处理；日志用 `spdlog-rs`
- **TypeScript/Svelte**: ESLint + Prettier 配置在 `app/` 下；严格模式
- **Prompt 模板**: 存为 `.prompt` 文件（见 `crates/intelligent/src/article_processor/`），不内嵌在 Rust 代码中
- **i18n**: 新增文本必须添加到 `app/src/lib/i18n/locales/zh.json` 和 `en.json`

---

## 3. 架构关键点

- **唯一入口**: 所有后端能力通过 `feed_api_rs::FeaturesAPI` trait 访问，不要直接调用子 crate
- **前端调用链**: Svelte 组件 → `app/src/lib/hybrid-apis/`（Tauri 命令绑定）→ `tauri-plugin-feed-api` → `feed_api_rs`
- **Provider 枚举派发**: LLM 和搜索 Provider 使用 enum 而非 `dyn Trait`，新增 Provider 需修改枚举和 match
- **用户数据全部本地**: SQLite 数据库，无远程同步，`recorder` crate 负责持久化

---

## 4. 外部依赖查询：使用 DeepWiki MCP

当遇到**不了解的外部依赖**时，**必须**使用 DeepWiki MCP 工具查询，不要凭猜测工作。

### 可用工具

| 工具 | 用途 | 示例 |
|------|------|------|
| `deepwiki:read_wiki_structure` | 查看仓库文档结构 | `tauri-apps/tauri` |
| `deepwiki:read_wiki_contents` | 阅读仓库文档全文 | `SeaQL/sea-orm` |
| `deepwiki:ask_question` | 对仓库提问具体问题 | `"sveltejs/kit", "SvelteKit 如何配置静态适配器？"` |

### 何时使用

- **必须使用**: 修改涉及你不熟悉的第三方库 API 或行为时
- **推荐使用**: 涉及 Tauri 插件开发、SeaORM 实体定义、SvelteKit 路由/加载等框架特定功能
- **无需使用**: 标准 Rust/TypeScript 语法、项目自身代码逻辑

### 本项目关键外部依赖的 DeepWiki 仓库名

| 依赖 | DeepWiki 仓库名 |
|------|-----------------|
| Tauri | `tauri-apps/tauri` |
| SvelteKit | `sveltejs/kit` |
| Svelte | `sveltejs/svelte` |
| Skeleton UI | `skeletonlabs/skeleton` |
| SeaORM | `SeaQL/sea-orm` |
| reqwest | `seanmonstar/reqwest` |
| tokio | `tokio-rs/tokio` |
| scraper | `causal-agent/scraper` |
| spdlog-rs | `SergioBenitez/spdlog-rs` |
| svelte-i18n | `kaisermann/svelte-i18n` |

### 使用示例

```
# 想了解 Tauri 2.x 插件如何注册命令
deepwiki:ask_question(repoName="tauri-apps/tauri", question="How to register custom commands in a Tauri 2.x plugin?")

# 想了解 SeaORM 的 Entity 宏定义
deepwiki:ask_question(repoName="SeaQL/sea-orm", question="How to define an entity with DeriveEntityModel and relations?")
```

---

## 5. 新增功能检查清单

添加新功能时，确保：

- [ ] 后端逻辑通过 `FeaturesAPI` trait 暴露（`crates/feed_api_rs/src/features/api.rs`）
- [ ] 前端通过 `hybrid-apis/` 调用 Tauri 命令，不直接 import Rust crate
- [ ] 如涉及新 LLM/Search Provider，修改对应枚举和 match 分支
- [ ] 如涉及新用户可见文本，同步更新 `zh.json` 和 `en.json`
- [ ] 如涉及非显而易见的设计决策，更新 `.ai-context/references/DECISIONS.md`
