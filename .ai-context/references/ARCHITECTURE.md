# Architecture

> Last updated: 2026-06-07

## System Diagram

```
┌──────────────────────┐       ┌──────────────────────────┐
│     Frontend         │       │       Backend (Rust)      │
│  SvelteKit + Tauri   │◀─────▶│                          │
├──────────────────────┤  Tauri ├──────────────────────────┤
│ routes/              │ Cmds  │ tauri-plugin-feed-api     │
│   main/  feeds/      │       │        │                 │
│   settings/ about/   │       │        ▼                 │
│ lib/                 │       │   feed_api_rs (Facade)   │
│   i18n/ widgets/     │       │     │    │    │    │      │
│   windows/ utils/    │       │  scrap llm intelligent recorder│
│   hybrid-apis/       │       │     │    │                │
└──────────────────────┘       │  search providers  ollama│
                               │  rss   article_proc     │
                               └──────────────────────────┘
                                         │
                                         ▼
                                   ┌──────────┐
                                   │  SQLite  │
                                   │ (SeaORM) │
                                   └──────────┘
```

## Component Responsibilities

| Crate | Responsibility |
|-------|---------------|
| `feed_api_rs` | Core facade API — orchestrates scrap, llm, intelligent, recorder, ollama |
| `tauri-plugin-feed-api` | Tauri plugin exposing `FeaturesAPI` as tauri commands to frontend |
| `scrap` | Internet data retrieval — search engines (Bing, Baidu) and RSS |
| `llm` | LLM abstraction layer — provider-agnostic interface for cloud/local models |
| `intelligent` | Article processing pipeline — prompt templates + optimization workflows |
| `recorder` | Local persistence — SQLite via SeaORM, stores articles & user config |
| `ollama` | Local Ollama management — download, launch, status check |
| `types` | Shared types — `Article`, `AppConfig`, `FeedsPackage`, LLM provider configs |

## Data Flow: Article Retrieval

```
User keywords → scrap (search/RSS) → raw articles → intelligent (LLM summarize/translate)
    → recorder (persist to SQLite) → frontend (display)
```

## Key Patterns

- **Facade pattern**: `feed_api_rs::FeaturesAPI` trait is the single entry point; `impl_default` provides the concrete implementation
- **Provider pattern**: LLM and search each define a provider trait (`IProvider`, `LLMProvider`), with enum dispatch for runtime selection
- **Hybrid runtime state**: `HybridRuntimeState` shared via Tauri state management (`Arc`-based)
- **Daemon mode**: Background scheduled feed updates via `app/src-tauri/src/daemon/`

## Frontend Architecture

| Path | Purpose |
|------|---------|
| `app/src/routes/` | SvelteKit page routes |
| `app/src/lib/hybrid-apis/` | Tauri command bindings (frontend → backend) |
| `app/src/lib/widgets/` | Reusable Svelte UI components |
| `app/src/lib/i18n/` | svelte-i18n setup, locales in `locales/` |
| `app/src/lib/windows/` | Window management utilities |

## LLM Provider Matrix

| Provider | File | Type |
|----------|------|------|
| GLM (智谱) | `llm_glm.rs` | Cloud |
| OpenAI-compatible | `llm_openaibase_like.rs` | Cloud |
| Mistral | `llm_mistral.rs` | Cloud |
| Platform | `llm_platform.rs` | Cloud |
| Ollama | `llm_ollama.rs` | Local |

## Search Provider Matrix

| Provider | File | Status |
|----------|------|--------|
| Bing | `bing.rs` | Default |
| Baidu | `baidu.rs` | Available |
