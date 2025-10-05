<script lang="ts">
	import { disableContextMenu } from '$lib/utils/dom';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';
	import FeedsList from './widgets/FeedsList.svelte';
	import ArticlesList from './widgets/ArticlesList.svelte';
	import SearchBar from './widgets/SearchBar.svelte';
	import ArticleReader from './widgets/ArticleReader.svelte';
	import Footer from './widgets/Footer.svelte';
	import ReaderBlankIndicator from './widgets/ReaderBlankIndicator.svelte';
	import { createStore } from './stores/index.svelte';
	import AISpritePanel from './widgets/AISpritePanel.svelte';
	import { globalToaster, spriteToaster } from './stores/toast';

	let mainStore = createStore();
    let feedsStore = mainStore.feeds;
    let articlesStore = mainStore.articles;
    let articlesListStore = articlesStore.list;
    let articlesSearchStore = articlesStore.search;
    let readerStore = mainStore.reader;
    let spriteStore = mainStore.sprite;

    // 用于存储FeedsList组件的展开函数引用
    let expandFeedGroupRef: ((feedId: string) => void) | undefined;

    function handleFeedTagClick(feedName: string) {
        // 根据feed名称找到对应的feed ID
        const feedId = feedsStore.findFeedIdByName(feedName);
        if (feedId) {
            // 先展开对应的feed组
            if (expandFeedGroupRef) {
                expandFeedGroupRef(feedId);
            }
            // 然后切换到对应的feed
            mainStore.setCurrentFeedId(feedId);
        }
    }
</script>

<div class="flex w-screen h-screen overflow-hidden flex-col">
	<div class="flex h-full flex-row overflow-hidden">
		<!-- Left Sidebar. -->
		<aside
			class="flex h-full w-[16rem] flex-col overflow-scroll-hidden pl-4 pt-4 pb-4 preset-filled-surface-50-950"
		>
			<FeedsList
                store={feedsStore}
                selectedFeedId={mainStore.currentFeedId}
                onFeedPressed={mainStore.setCurrentFeedId}
                onSelectToday={mainStore.onSelectToday}
                onSelectWeekend={mainStore.onSelectWeekend}
                isTodaySelected={mainStore.isTodaySelected}
                isWeekendSelected={mainStore.isWeekendSelected}
                onSelectFavorite={mainStore.onSelectFavorite}
                isFavoriteSelected={mainStore.isFavoriteSelected}
                onSelectUnread={mainStore.onSelectUnread}
                isUnreadSelected={mainStore.isUnreadSelected}
                onExpandFeedGroup={(fn) => { expandFeedGroupRef = fn; }}
            />
		</aside>
		<aside
			use:disableContextMenu
			class="flex h-full w-[20rem] flex-col overflow-scroll-hidden p-4 preset-filled-surface-50-950"
		>
			<SearchBar store={articlesStore.search} articles_store={articlesListStore} />
			<hr class="hr" />
			<ArticlesList
				store={articlesListStore}
				markAsRead={readerStore.markAsRead}
				onArticlePressed={mainStore.setCurrentArticle}
				selectedArticle={mainStore.currentArticle}
				isFilterActived={articlesSearchStore.isFilterActived}
				isFeedSpecified={mainStore.isFeedSpecified}
			/>
		</aside>
		<!-- Main Content -->
        <main class="h-full flex-1 preset-filled-surface-100-900">
            {#if mainStore.currentArticle}
                <ArticleReader 
                    articleId={mainStore.currentArticle.id} 
                    store={readerStore}
                    onFeedTagClick={handleFeedTagClick}
                />
            {:else}
                <ReaderBlankIndicator />
            {/if}
        </main>
	</div>
	<!-- Footer -->
	<Footer tasksStore={mainStore.tasks} />

	<AISpritePanel store={spriteStore} />

	<!-- In-App Notifications -->
	<Toaster toaster={globalToaster} />
	<Toaster toaster={spriteToaster} />
</div>
