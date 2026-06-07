# Design Decisions

> Last updated: 2026-06-07

## ADR-1: Facade Pattern for Core API

**Context:** Multiple subsystems (scrap, llm, intelligent, recorder, ollama) need coordinated access.

**Decision:** `feed_api_rs` implements a Facade pattern via `FeaturesAPI` trait.

**Rationale:** Single entry point reduces coupling; trait allows test mocking; concrete impl in `impl_default.rs` orchestrates all subsystems.

**Trade-off:** Adding new features requires modifying both the trait and its implementation, but this is intentional — all capabilities flow through one API surface.

---

## ADR-2: Rust Monorepo with Workspace Crates

**Context:** Backend logic spans multiple domains (scraping, LLM, storage, intelligence).

**Decision:** Use Cargo workspace with dedicated crates per domain.

**Rationale:** Clear ownership boundaries; each crate compiles independently; `types` crate prevents circular dependencies; `feed_api_rs` depends on all others as the facade.

**Trade-off:** More `Cargo.toml` files to maintain, but dependency management is explicit and build caching is better.

---

## ADR-3: Dual Provider Enum Dispatch (No dyn Trait)

**Context:** LLM and search providers need runtime selection based on user config.

**Decision:** Use enums (`ScrapProviderEnums`, LLM provider dispatch) instead of `dyn Trait`.

**Rationale:** No dynamic dispatch overhead; exhaustive match ensures all providers handled; simpler for a small, known set of providers.

**Trade-off:** Adding a new provider requires modifying the enum and match arms, but the provider set is small and stable.

---

## ADR-4: SeaORM + SQLite for Local Storage

**Context:** Articles and user config must persist locally with zero server dependency.

**Decision:** Use SeaORM with SQLite backend.

**Rationale:** Async ORM compatible with Tokio; SQLite requires no server; type-safe queries; migration support.

**Trade-off:** SQLite concurrency limits (WAL mode helps), but this is a single-user desktop app — not a concern.

---

## ADR-5: Prompt Templates as .prompt Files

**Context:** Article processing requires carefully tuned LLM prompts.

**Decision:** Store prompts as `.prompt` files in `crates/intelligent/src/article_processor/`.

**Rationale:** Prompts are content, not logic — separating them from Rust code enables non-developer editing and clear version tracking.

**Trade-off:** No compile-time validation of prompt structure, but runtime errors are acceptable for template content.

---

## ADR-6: macOS Close-to-Tray Behavior

**Context:** Desktop app should stay running for scheduled feed updates.

**Decision:** On macOS, window close is intercepted → app hides instead of quitting.

**Rationale:** Daemon mode needs the process alive for background updates; users expect macOS apps to stay in dock/menu bar.

**Trade-off:** Users must explicitly quit via tray icon or Cmd+Q.
