# AI处理域技术实现文档

## 概述

AI处理域是Saga Reader系统的核心智能引擎，负责对从外部数据源抓取的原始文章进行智能化预处理与增强，通过三种核心模式——净化（Purge）、优化（Optimizer）和融合（Melt）——显著提升信息的可读性、专业性与价值密度。该模块采用Rust异步编程模型，基于LLM代理服务构建，实现了高并发、低延迟的本地化AI处理能力，是系统“全本地化、无云端依赖”架构的关键支柱。

## 核心架构与设计模式

AI处理域采用**责任链模式**与**工厂模式**相结合的架构，实现了模块化、可扩展的处理流水线。

1.  **统一接口（IArticleProcessor）**：在 `types.rs` 中定义了核心的 `IArticleProcessor` trait，规定了所有文章处理器必须实现的 `process` 方法。该方法接收一个不可变的 `Article` 引用和一个 `LLMInstructOption` 配置选项，返回一个 `Future`，其结果为处理后的 `Article` 对象。此接口确保了Purge、Optimizer、Melt等不同处理器在框架层面可以被统一调度和调用，形成清晰的处理流水线。

2.  **预设处理器工厂（IPresetArticleLLMProcessor）**：在 `llm_processor.rs` 中定义了 `IPresetArticleLLMProcessor` trait。该trait的 `new_processor` 方法是一个工厂方法，用于根据传入的 `LLMSection` 配置（如模型名称、API地址）创建并初始化一个具体的 `ArticleLLMProcessor` 实例。Purge、Optimizer、Melt三个模块分别实现了此trait，它们的职责不再是直接处理文本，而是**定义其专属的系统提示词（System Prompt）和用户指令（User Prompt）**，并配置LLM的生成参数（如温度、上下文长度）。

3.  **核心处理器（ArticleLLMProcessor）**：这是实际执行AI处理的引擎。它封装了一个 `CompletionAgent` 实例，该实例负责与底层的Ollama、GLM等LLM服务进行通信。`ArticleLLMProcessor` 的核心逻辑是将原始文章内容、预设的系统提示词、用户指令以及语言偏好（通过 `LLMInstructOption` 传入）拼接成一个完整的、结构化的Prompt，然后调用 `CompletionAgent.completion()` 发起异步请求，最终将LLM返回的处理结果写入 `Article` 对象的 `content` 字段。

## 模块实现细节

### 1. Purge（噪声移除）
*   **目标**：从原始HTML或富文本中移除广告、导航栏、页脚、评论区等无关内容，仅保留核心正文与图片。
*   **实现**：
    *   **系统提示词** (`purge_sys.prompt`)：定义了“网页源码分析师”的角色，指令明确要求移除JavaScript、CSS、`<header>`、`<footer>`、`<nav>`等标签，并提取`<p>`、`<h1-6>`中的文本和`<img>`、`<picture>`中的图片链接，最终以Markdown格式输出。
    *   **用户指令** (`purge_suffix.prompt`)：简洁明了，仅要求“提取上述源码中的正文内容的正文与图片链接，以markdown格式输出给我”。
    *   **LLM配置**：设置 `num_ctx: Some(8192)`，确保能处理较长的网页源码。
*   **输出**：一个干净、结构化的Markdown文档，包含正文段落和嵌入的图片链接。

### 2. Optimizer（语言优化）
*   **目标**：将净化后的文本进行语言润色，使其更精炼、专业、符合现代阅读习惯，尤其针对邮件或报告场景。
*   **实现**：
    *   **系统提示词** (`optimizer_sys.prompt`)：这是一个高度复杂的提示词，定义了“专业内容设计师”的角色。它不仅要求将文本转换为“现代HTML邮件片段”，还详细规定了必须使用的HTML模板（标准段落、关键点列表、强调文本、引用块、图片块），并强调了视觉美学、一致性、避免冗余、翻译为中文等严格要求。其核心是**将文本内容转化为一个视觉上优雅、信息密度高的HTML片段**。
    *   **用户指令** (`optimizer_suffix.prompt`)：为空。这意味着优化过程完全依赖于系统提示词的完整指令，用户指令仅作为上下文传递原始文本。
    *   **LLM配置**：设置 `temperature: Some(0.1)`，使输出更加稳定、保守，避免过度发挥。
*   **输出**：一个符合特定HTML邮件模板的、视觉优化的纯HTML代码片段，而非纯文本。

### 3. Melt（多文融合）
*   **目标**：将多篇关于同一主题的文章进行综合、对比与提炼，生成一份结构化的、具有洞察力的摘要。
*   **实现**：
    *   **系统提示词** (`melt_sys.prompt`)：定义了“擅长多个行业领域的专家”角色，要求按照固定的五个部分进行结构化总结：内容概述、关键信息提炼、文章质量及疑点、指导建议、专业术语说明。
    *   **用户指令** (`melt_suffix.prompt`)：明确要求以“第三方评价者”的视角，按照上述五个部分进行总结，并特别指出“专业术语说明”部分必须使用Markdown表格格式。它还要求输出为“白话”，即通俗易懂的语言。
    *   **LLM配置**：设置 `temperature: Some(0.7)`，允许模型进行一定程度的创造性综合与观点提炼。
*   **输出**：一个遵循严格结构的Markdown文档，包含清晰的二级标题和表格，是对多篇文章的深度整合与评价。

## 与LLM代理服务的交互

AI处理域的所有处理逻辑都通过 `ArticleLLMProcessor` 与 `CompletionAgent` 交互。`CompletionAgent` 是一个统一的LLM调用抽象层，位于 `crates/llm` 模块中。它根据 `app_config.toml` 中的 `active_provider_type`（如 `Ollama`）动态选择并调用对应的实现（如 `OllamaCompletionService`），向本地运行的Ollama服务发送HTTP POST请求（`/api/generate`），传递构建好的Prompt。这种设计使得AI处理域与具体的LLM提供商解耦，未来可以轻松扩展对GLM、Mistral等其他本地模型的支持。

## 总结

AI处理域的设计体现了“**配置即功能**”的优秀工程实践。三个核心处理器（Purge, Optimizer, Melt）通过简单的结构体实现，其真正的“智能”来源于外部的提示词文件（`.prompt`）。这种设计使得：

1.  **逻辑与内容分离**：业务逻辑（`llm_processor.rs`）稳定不变，处理效果的调整只需修改提示词文件，无需重新编译代码。
2.  **易于调试与迭代**：开发者可以独立测试和优化每一个提示词，快速迭代处理效果。
3.  **高度专业化**：每个提示词都经过精心设计，赋予LLM一个明确的角色和任务，确保输出质量。

该模块成功地将复杂的AI能力封装为一个可复用、可扩展的引擎，为Saga Reader的“智能阅读”核心价值提供了坚实的技术基础。