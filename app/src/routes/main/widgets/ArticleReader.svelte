<script lang="ts">
  import { _ } from "svelte-i18n";
  import { Tabs } from "@skeletonlabs/skeleton-svelte";
  import {
    Link,
    ExternalLink,
    Highlighter as IconMelt,
    Computer as IconOptimize,
    Star as IconStar,
    StarOff as IconStarOff,
    Paperclip as IconOriginal,
  } from "lucide-svelte";
  import ArticleRenderWidget from "$lib/widgets/ArticleRenderWidget.svelte";
  import EmbedWebView from "$lib/widgets/EmbedWebView.svelte";
  import type { ArticleReaderProps, ArticleReadMode } from "./types";
  import type { Article } from "$lib/types/article";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  import { featuresApi } from "$lib/hybrid-apis/feed/impl";
  import { disableContextMenu } from "$lib/utils/dom";
  import { globalToaster as toaster } from "../stores/toast";

  let group: ArticleReadMode = $state("optimized");

  let { articleId, store, onFeedTagClick }: ArticleReaderProps = $props();

  let article: Article | null = $state(null);
  let articleUpdatedSeq: number = $state(0);

  async function copyLink() {
    if (!article) return;
    await writeText(article.source_link);
    toaster.info({
      description: $_("reader.tip_link_copyed"),
    });
  }

  function openOriginalPage() {
    if (!article) return;
    featuresApi.open_article_external(article.source_link);
  }

  function refreshByEnhancedScraper() {
    if (!article) return;
    store
      .refreshByEnhancedScraper(article.id, article.source_link)
      .then((r) => {
        if (r.id !== article?.id) return;
        articleUpdatedSeq += 1;
        article = r;
      })
      .catch((e) => console.error("refreshByEnhancedScraper", e));
  }

  function switchStar(article: Article) {
    if (!article) return;
    const is_favorite_new = !article.is_favorite;
    featuresApi
      .set_favorite(article.id, is_favorite_new)
      .then(() => (article.is_favorite = is_favorite_new))
      .catch((e) => console.error("reader.article switchStar failured", e));
  }

  function handleFeedTagClick(feedName: string | undefined) {
    if (!feedName || !onFeedTagClick) return;
    onFeedTagClick(feedName);
  }

  $effect(() => {
    featuresApi
      .query_by_id(articleId)
      .then((queried_article) => {
        article = queried_article;
      })
      .catch((e) => console.error("reader.article query failured", e));
  });
</script>

<div class="flex h-full w-full flex-col p-4">
  <div
    use:disableContextMenu
    class="card flex min-h-24 flex-col p-4 preset-filled-surface-50-950"
  >
    {#if article !== null}
      <div class="flex flex-row items-center">
        <h5 class="h5 flex-1">{article.title}</h5>
        <div class="flex flex-row">
          <button
            class="btn hover:bg-surface-200-800 rounded-full w-12 h-12"
            onclick={() => switchStar(article!)}
            onkeypress={() => switchStar(article!)}
          >
            {#if article.is_favorite}
              <IconStarOff size={20} />
            {:else}
              <IconStar size={20} />
            {/if}
          </button>
          <button
            type="button"
            class="btn w-12 h-12 rounded-full preset-filled"
            onclick={copyLink}><Link size={16} /></button
          >
          <button
            type="button"
            class="btn w-12 h-12 ml-2 rounded-full preset-filled"
            onclick={openOriginalPage}><ExternalLink size={16} /></button
          >
        </div>
      </div>

      <!-- 文章元信息 -->
      <div class="flex items-center gap-3 mt-3 text-sm text-surface-600-300">
        <span>{new Date(article.published_at).toLocaleDateString()}</span>

        <!-- Feed来源标签 -->
        {#if article.feed_name}
          <button
            class="feed-tag"
            onclick={() => handleFeedTagClick(article.feed_name)}
            onkeypress={() => handleFeedTagClick(article.feed_name)}
          >
            <svg
              class="feed-icon"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <path
                d="M6.503 20.752c0 1.794-1.456 3.248-3.251 3.248S0 22.546 0 20.752s1.456-3.248 3.252-3.248 3.251 1.454 3.251 3.248zm-6.503-12.572v4.811c6.05.062 10.96 4.966 11.022 11.009h4.817c-.062-8.71-7.118-15.758-15.839-15.82zm0-3.368c10.58.046 19.152 8.594 19.183 19.188h4.817c-.03-13.231-10.755-23.954-24-24v4.812z"
              />
            </svg>
            {article.feed_name}
          </button>
        {/if}
      </div>
    {/if}
  </div>

  <div
    class="mt-4 flex-1 overflow-hidden rounded pt-2 preset-filled-surface-50-950"
  >
    <Tabs
      value={group}
      onValueChange={(e) => (group = e.value as ArticleReadMode)}
      listJustify="justify-center"
      classes="h-full overflow-hidden flex flex-col"
      listClasses="flex-0"
      contentClasses="h-full flex-1 overflow-auto"
      listMargin="p-0"
    >
      {#snippet list()}
        <Tabs.Control classes="border-b-4" value="optimized">
          {#snippet lead()}<IconOptimize size={20} />{/snippet}
          {`${$_("reader.tab_optimized_content")}`}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
        </Tabs.Control>
        <Tabs.Control classes="border-b-4" value="melted">
          {#snippet lead()}<IconMelt size={20} />{/snippet}
          {`${$_("reader.tab_melted_content")}`}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
        </Tabs.Control>
        <Tabs.Control classes="border-b-4" value="original">
          {#snippet lead()}<IconOriginal size={20} />{/snippet}
          {`${$_("reader.tab_melted_original")}`}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
        </Tabs.Control>
      {/snippet}
      {#snippet content()}
        {#if article}
          <Tabs.Panel value="optimized">
            {#key `optimized-${article.id}-${articleUpdatedSeq}`}
              <ArticleRenderWidget value={article.optimized_content} />
            {/key}
          </Tabs.Panel>
          <Tabs.Panel value="melted">
            {#key `melted-${article.id}-${articleUpdatedSeq}`}
              <ArticleRenderWidget value={article.melted_content} />
            {/key}
          </Tabs.Panel>
          <Tabs.Panel value="original" classes="h-full">
            <EmbedWebView src={article.source_link} />
          </Tabs.Panel>
        {/if}
      {/snippet}
    </Tabs>

    <div class="absolute right-16 bottom-16">
      <button
        type="button"
        onclick={refreshByEnhancedScraper}
        class="btn preset-filled-primary-500 rounded-full">R</button
      >
    </div>
  </div>
</div>

<style>
  .feed-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background-color: rgb(59 130 246);
    color: white;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .feed-tag:hover {
    background-color: rgb(37 99 235);
  }

  .feed-icon {
    opacity: 0.8;
  }
</style>
