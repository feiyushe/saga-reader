<script lang="ts">
    import { _ } from 'svelte-i18n';
	import { Status } from '../stores/loading.svelte';
	import type { ArticlesListProps } from './types';
	import type { ArticlesGroup } from './types';
	import { Progress } from '@skeletonlabs/skeleton-svelte';
	const {
		store,
		selectedArticle,
		onArticlePressed,
		isFilterActived,
		markAsRead,
		isFeedSpecified
	}: ArticlesListProps = $props();
	const { articles_init_loading, articles_continous_loading, loadMore, refresh, updateFeeds } =
		store;

	let scroller: HTMLDivElement;

	async function onloadFeedContents() {
		await updateFeeds();
		await refresh(false);
	}

	$effect(() => {
		console.log(
			`the articles list's associatedFeedId has been changed to ${store.associatedFeedId}`
		);
		scroller?.scrollTo(0, 0);
	});

	$effect(() => {
		if (!selectedArticle) return;
		if (!selectedArticle.has_read) {
			markAsRead(selectedArticle.id).then(() => (selectedArticle.has_read = true));
		}
	});
</script>

{#if articles_init_loading.status === Status.Loading}
	<Progress
		classes="mb-2 mt-2 w-full"
		value={null}
		meterAnimate="articles_nav_loading_indicator"
		meterBg="bg-primary-500"
	/>
{:else if store.articles_init_loading.status === Status.Completed}{:else if store.articles_init_loading.status === Status.Error}
	<button type="button" class="btn preset-filled" onclick={store.updateFeeds}>
	   {$_('main.articles.error_retry')}
	</button>
{/if}

{#if store.articles_init_loading.status === Status.Completed}
	<!-- 当加载状态为Completed状态时，显示列表 -->
	{@render render_articles_list(isFilterActived ? store.filteredArticles : store.groupedArticles)}
{:else if store.groupedArticles.length !== 0}
	<!-- 当加载状态为非Completed状态时，如果有数据也显示列表，用以支持加载中不足端浏览已有的订阅内容 -->
	{@render render_articles_list(isFilterActived ? store.filteredArticles : store.groupedArticles)}
{/if}

{#snippet render_articles_list(groups: ArticlesGroup[])}
	<div class="flex flex-1 h-full flex-col overflow-y-auto overflow-x-clip" bind:this={scroller}>
		{#each groups as { name, articles } (name)}
			<h6 class="mt-4 font-bold">{name}</h6>
			{#each articles as article (article.id)}
				<div class="mb-2 mt-2">
					<!-- svelte-ignore a11y_invalid_attribute -->
					<a
						class={`flex flex-row card ${selectedArticle?.id === article.id ? 'preset-filled-surface-900-100' : 'preset-filled-surface-100-900'}`}
						href='#'
						onclick={() => onArticlePressed(article)}
					>
						{#if !article.has_read && selectedArticle?.id !== article.id}
							<div class="h-10 w-0.5 mt-4 mr-3 preset-filled-primary-500"></div>
						{/if}

						<article
                            class={`hover:scale-105 transition-transform flex-1 pt-4 pb-4 pr-4 ${article.has_read || selectedArticle?.id == article.id ? 'pl-4' : ''}`}
                        >
                            <h6 class="font-bold">{article.title}</h6>

                            <p
                                class={`line-clamp-2 ${selectedArticle?.id === article.id ? 'text-surface-400-600' : 'text-surface-600-400'}`}
                            >
                                {article.head_read}
                            </p>
                            
                            <!-- 文章元信息 -->
                            <div class="flex items-center justify-between mt-2 text-xs text-surface-500">
                                <span>{new Date(article.published_at).toLocaleDateString()}</span>
                                
                                <!-- Feed来源标签 -->
                                {#if article.feed_name}
                                    <span class="feed-tag-small">
                                        {article.feed_name}
                                    </span>
                                {/if}
                            </div>
                        </article>
					</a>
				</div>
			{/each}
		{/each}
		{#if !isFilterActived}
			{#if store.groupedArticles.length !== 0}
				<hr class="hr mb-3" />
				{#if articles_continous_loading.status === Status.Completed}
					<button type="button" class="btn preset-filled" onclick={loadMore}>{$_('main.articles.tip_click_to_load_more')}</button>
				{:else}
					<button type="button" class="btn preset-filled">{$_('main.articles.tip_loading')}</button>
				{/if}
			{:else if !isFeedSpecified}
				{#if articles_init_loading.status !== Status.Loading}
					<button type="button" class="btn preset-filled" onclick={onloadFeedContents}
						>{$_('main.articles.tip_empty_try_again')}</button
					>
				{/if}
			{:else}
				<div class="flex flex-col p-4 w-full items-center">
					<p class="text-surface-500">{$_('main.articles.tip_empty')}</p>
				</div>
			{/if}
		{/if}
	</div>
{/snippet}

<style>
    :global(.articles_nav_loading_indicator) {
        animation: my-custom-animation 2s ease-in-out infinite;
    }
    @keyframes my-custom-animation {
        0% {
            translate: -100%;
        }
        50% {
            scale: 1;
        }
        100% {
            translate: 200%;
        }
    }
    
    .feed-tag-small {
        padding: 0.125rem 0.375rem;
        background-color: rgb(59 130 246);
        color: white;
        border-radius: 0.25rem;
        font-size: 0.625rem;
        font-weight: 500;
        opacity: 0.8;
    }
</style>
