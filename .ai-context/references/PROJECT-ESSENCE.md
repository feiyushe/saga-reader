# Project Essence

> Last updated: 2026-06-07

## What Is This?

Saga Reader is an AI-driven think-tank style reader that automatically retrieves, summarizes, and translates internet content based on user-defined interest keywords.

## Why Does It Exist?

Existing RSS readers and news aggregators require manual curation, lack intelligent summarization, and often compromise user privacy. Saga Reader solves this by providing a fully automated, privacy-first, AI-enhanced reading experience.

## Who Is It For?

Information enthusiasts who want personalized content without ads, tracking, or manual curation. Users range from casual readers to researchers needing multi-language content.

## Key Value Proposition

- **Interest-based auto-retrieval** — Define keywords, get articles from the whole internet
- **AI companion reading** — Chat with AI about what you're reading in real-time
- **Multi-model LLM support** — Cloud (GLM, OpenAI, Mistral) and local (Ollama) models
- **Privacy-first** — All data stored locally on user's machine, no third-party tracking
- **Extremely lightweight** — <10MB memory, runs on old hardware

## Core Constraints

- Cross-platform desktop: Windows, macOS, Linux
- All user data must remain local (SQLite via SeaORM)
- No third-party analytics or tracking
- Must support both cloud and local LLM inference
- i18n: Chinese and English (extensible via `app/src/lib/i18n/locales/`)

## Monorepo Structure

| Area | Tech |
|------|------|
| Desktop app | Tauri 2.x (Rust) + SvelteKit 2.x |
| Package manager | Bun (recommended) |
| Rust workspace | `Cargo.toml` at root, crates in `crates/` |
| Frontend workspace | Bun workspaces, app in `app/` |
