
<p align="center">
  <img width="120" height="120" src="https://aiqino.netlify.app/favicon.png">
</p>
<h3 align="center">Saga Reader</h3>
<p align="center">Your AI-Powered think tank assistant</p>
<p align="center">
  <img src="https://img.shields.io/github/v/release/sopaco/saga-reader?label=version" />
  <img src="https://img.shields.io/github/downloads/sopaco/saga-reader/total" />
</p>
<hr />

<p align="center">
    <a href="./README.md">English</a>
    |
    <a href="./README_zh.md">中文</a>
</p>

> 🚀 Help me develop this software better by [sponsoring on GitHub](https://github.com/sponsors/sopaco)


## What's Saga Reader
Saga Reader is an AI-driven think tank-style reader that automatically retrieves information from the internet based on user-specified topics and preferences. It uses cloud or local large models to summarize and provide guidance, and it includes an AI-driven interactive companion reading function, allowing you to discuss and exchange ideas with AI about the content you've read.

Saga Reader is completely free and open-source, meaning all data is securely stored on your own computer and is not controlled by third-party service providers. Additionally, you can manage your subscription keywords based on your interests and preferences without being disturbed by advertisements and commercialized content.

Download it from [website](https://aiqino.netlify.app)!

## Snapshots

---

![snapshot-glance](./uprise/snapshot-glance.webp)

## How it works

---

![archi-diags-en.png](./uprise/archi-diags-en.png)

## Features

### Content Subscriptions
- Flexibly set your content subscriptions based on your interests. Simply define a few keywords to automatically gather information from the global internet.

### Translation
- Automatically translate foreign language information. You can have articles in other languages automatically translated into your preferred language.

### Security and Privacy
- Offers the best security and privacy features. Your data is stored entirely on your personal computer and is not tracked or influenced by third-party service providers.

### Model Support
- Supports connection to cloud-based large models or local large models on your personal computer for inference.

### Performance
- Extremely lightweight and high-performing, this application is developed using Rust and Svelte technologies, boasting the lowest memory and CPU consumption among its peers, with memory usage below 10MB. You can deploy it on any old device.

### UI Design
- Clean UI design, lightweight, practical, and easy to use. You won't be bothered by annoying ads or commercial elements.

### Article Viewer
- Use the built-in article viewer to read full content, or switch to your preferred browser for browsing.

### Search and Background Updates
- Supports article search.
- Can silently update content in the background. Thanks to Rust technology, it boasts an extremely small memory footprint of just 10MB and excellent running performance.

## Development
### Prerequisites
- [**Rust**](https://www.rust-lang.org)
- [**Bun**](https://bun.sh) (recommended) or NodeJS


### Installation
The installation is straight forward, just follow the steps below:
<br>
1. Clone the repository.
    ```sh
    git clone https://github.com/sopaco/saga-reader.git --recursive
    ```
2. Step into the cloned project folder.
    ```sh
    cd saga-reader
    ```
3. Install Dependencies.

    *Recommend **[bun](https://bun.sh)**, this is blazing fast.*
    ```sh
    # **recommend, this is blazing fast**
    bun install

    # or use pnpm
    # pnpm install

    # or use npm
    # npm install
    ```
4. Run

    ```sh
    # **recommend, this is blazing fast**
    bun run dev

    # or use pnpm
    # pnpm run dev

    # or use npm
    # npm run dev
    ```
5. Or build it directly

> By default, it will build for the version of the system used by the machine you are compiling on. If you need to cross-compile, you can run the `build:macos` or `build:windows` scripts in the `package.json`.
    ```sh
    # **recommend, this is blazing fast**
    bun run build

    # or use pnpm
    # pnpm run build

    # or use npm
    # npm run build
    ```
<br>

### Monorepo App Architecture

We use a combination of Rust, Svelte (SvelteKit), Tauri, SeaORM, SqlLite, TailwindCSS throughout this monorepo.

#### App

- `desktop`: A [Tauri](https://tauri.app) (Rust) app, using [Svelte](https://svelte.dev/) on the frontend.

#### Packages / Crates

- `intelligent`: Article optimization workflow module, providing abstractions for article optimization processes and prompt engineering optimizations.
- `scrap`: Provides data scraping functionality, fetching internet information by calling mainstream search engines. This module is by default fully localized and does not rely on any third-party services.
- `recorder`: Provides local storage functionality, where user interest prompts, raw articles, and post-processed optimized articles are saved on the user's personal computer storage.
- `llm`: Provides internal LLM Provider abstractions and adapts to various cloud and local large model service implementations.
- `ollama`: Operates local ollama, including running basic instances, model updates, and management functions.
- `feed_api_rs`: The core capability API and implementation, based on the classic facade pattern.
- `tauri-plugin-feed-api`: Core capability API for frontend calls via tauri commands.
- `types`: Shared basic types module.

#### Schematic Diagram
```plaintext
Apply
+---------------------+     +---------------------+
|      Frontend       |     |     Backend         |
|  (Svelte/SvelteKit) |<--->|  (Rust Modules)     |
+---------------------+     +---------------------+
       ^   ^   ^                      |   |   |
       |   |   |                      |   |   |
+------+---+---+-------+       +------+---+---+-------+
|  UI/UX Components    |       |  tauri-plugin-feed-api|
|  State Management    |       |  feed_api_rs          |
|  Internationalization|       |  llm                  |
|  Styling (Tailwind)  |       |  ollama               |
|  Build Tools (Vite)  |       |  recorder             |
|                      |       |  scrap                |
|                      |       |  types                |
|                      |       |  intelligent          |
+----------------------+       +-----------------------+
```

### Contribute

Help improve Saga Reader by reporting bugs or submitting feature requests through [GitHub Issues](https://github.com/sopaco/saga-reader/issues).

Similarly, there are some features in wunderlist that need to be improved.

**For Example**

- You can help implement additional [Internet Search Providers](https://github.com/sopaco/saga-reader/tree/master/crates/scrap/src/search) beyond Bing, such as Google.
- You can assist in integrating more [Online LLM Providers](https://github.com/sopaco/saga-reader/tree/master/crates/llm/src/providers) besides GLM Flash, like OpenAI.
- You can also contribute to the internationalization of the app by providing [translations into additional languages](https://github.com/sopaco/saga-reader/tree/master/app/src/lib/i18n/locales). Refer to the [svelte-i18n](https://github.com/kaisermann/svelte-i18n/blob/main/docs/Getting%20Started.md#5-localizing-your-app) repository to get started with internationalization.

If you enjoy using this app, consider supporting its development by donating through [GitHub Sponsors](https://github.com/sponsors/sopaco), [Paypal](https://paypal.me/skyronj), or [Alipay](https://aiqino.netlify.app/uprise-assets/alipay.jpg).

### Developed with

- [Rust](https://github.com/rust-lang/rust)
- [Tauri](https://github.com/tauri-apps/tauri)
- [Svelte](https://github.com/sveltejs/svelte)
- [SvelteKit](https://github.com/sveltejs/kit)
- [Skeleton](https://github.com/skeletonlabs/skeleton)
- [sea-orm](https://github.com/SeaQL/sea-orm)

### License
MIT, A copy of the license is provided in the [LICENSE](./LICENSE) file.

### About Me

> 🚀 Help me develop this software better by [sponsoring on GitHub](https://github.com/sponsors/sopaco)

An experienced internet veteran, having navigated through the waves of PC internet, mobile internet, and AI applications. Starting from an individual mobile application developer to a professional in the corporate world, I possess rich experience in product design and research and development. Currently, I am employed at [Kuaishou](https://en.wikipedia.org/wiki/Kuaishou), focusing on the R&D of universal front-end systems and AI exploration.

WeChat: dokhell

Email: dokhell@hotmail.com
