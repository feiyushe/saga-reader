<script lang="ts">
    import { _ } from 'svelte-i18n';
	import { ask } from '@tauri-apps/plugin-dialog';
	import ContextMenuProvider from '$lib/widgets/ContextMenuProvider.svelte';
	import IconAdd from 'lucide-svelte/icons/plus';
	import IconPackageUnExpand from 'lucide-svelte/icons/folder';
	import IconPackageExpand from 'lucide-svelte/icons/folder-open';
	import IconAddFeed from 'lucide-svelte/icons/circle-plus';
	import IconWeekend from 'lucide-svelte/icons/package';
	import IconToday from 'lucide-svelte/icons/newspaper';
	import IconSettings from 'lucide-svelte/icons/settings';
	// import IconRssSource from 'lucide-svelte/icons/rss';
	import IconScrapSource from 'lucide-svelte/icons/globe';
	import IconFavorites from 'lucide-svelte/icons/file-heart';
	import IconUnread from 'lucide-svelte/icons/eye-off';
	import type { FeedsListProps } from './types';
	import { Status } from '../stores/loading.svelte';
	import { openSettings } from '$lib/windows/index';
	import type { FeedsPackage, FeedTargetDescription } from '$lib/hybrid-apis/feed/types';
	import {
		openFeedCreateWindow,
		openFeedEditWindow,
		openFeedPackageCreateWindow,
		openFeedPackageEditWindow
	} from '$lib/windows/lite-edit';
	import { disableContextMenu } from '$lib/utils/dom';
    import { format } from '$lib/utils/text';

	let {
		store,
		onFeedPressed,
		selectedFeedId,
		onSelectToday,
		onSelectWeekend,
		isTodaySelected,
		isWeekendSelected,
		onSelectFavorite,
		isFavoriteSelected,
		onSelectUnread,
		isUnreadSelected
	}: FeedsListProps = $props();
	const expandState: {
		[key: string]: boolean;
	} = $state({});

	function createRefreshFeedsAction(call: () => Promise<unknown>) {
		return async () => {
			await call();
			store.refresh();
		};
	}

	function createFeedPackageMenus(feedPackage: FeedsPackage) {
		const { id, name } = feedPackage;
		return [
			{
				name: 'new',
				onClick: createRefreshFeedsAction(async () => {
					await openFeedCreateWindow(id);
					expandGroup(id);
				}),
				displayText: $_('main.feeds.menu.actions.create'),
				class: ''
			},
			{
				name: 'edit',
				onClick: createRefreshFeedsAction(() => openFeedPackageEditWindow(id, name)),
				displayText: $_('main.feeds.menu.actions.edit'),
				class: ''
			},
			{
				name: 'del',
				onClick: createRefreshFeedsAction(async () => {
					const answer = await ask(
						format($_('main.feeds.menu.actions.delete_prompt'), {
						name,
						length: `${feedPackage.feeds.length}`
						}),
						{
							title: $_('main.feeds.menu.actions.delete_dialog_title'),
							kind: 'warning'
						}
					);
					if (!answer) return;
					await store.removeFeedsPackage(feedPackage.id);
				}),
				displayText: $_('main.feeds.menu.actions.delete'),
				class: ''
			}
		];
	}

	function createFeedMenus(feedPackage: FeedsPackage, feed: FeedTargetDescription) {
		return [
			{
				name: 'edit',
				onClick: createRefreshFeedsAction(() =>
					openFeedEditWindow(feed.id, feed.name, feed.fetcher_id, feed.data, feedPackage.id)
				),
				displayText: $_('main.feed.menu.actions.edit'),
				class: ''
			},
			{
				name: 'del',
				onClick: createRefreshFeedsAction(async () => {
					const answer = await ask(format($_('main.feed.menu.actions.delete_prompt'), {
					   name: feed.name
					}), {
						title: $_('main.feed.menu.actions.delete_dialog_title'),
						kind: 'warning'
					});
					if (!answer) return;
					await store.removeFeed(feedPackage.id, feed.id);
				}),
				displayText: $_('main.feed.menu.actions.delete'),
				class: ''
			}
		];
	}

	function toggleExpand(key: string) {
		expandState[key] = !expandState[key];
	}

	function expandGroup(key: string) {
		expandState[key] = true;
	}

	function onAddFeedPressed() {
		createRefreshFeedsAction(() => openFeedPackageCreateWindow())();
	}
</script>

{#snippet listSection(text: string)}
	<h5 class="h6 p-2 mt-4 cursor-default">{text}</h5>
{/snippet}

{#snippet listSectionWithAction(text: string, IconRender, onclick)}
	<div class="flex p-2 mt-4 justify-between items-center">
		<h5 class="h6">{text}</h5>
		<div
			class="hover:bg-surface-100-900 p-2"
			{onclick}
			onkeypress={onclick}
			role="button"
			tabindex="0"
		>
			<IconRender size={18} />
		</div>
	</div>
{/snippet}

{#snippet listItem(text: string, IconRender, onclick, stateSelected, menus)}
	<a href={'#'} {onclick}>
		{#if menus && menus.length !== 0}
			<ContextMenuProvider {menus}>
				{@render listItemInner(text, IconRender, stateSelected)}
			</ContextMenuProvider>
		{:else}
			{@render listItemInner(text, IconRender, stateSelected)}
		{/if}
	</a>
{/snippet}

{#snippet listItemInner(text: string, IconRender, stateSelected)}
	<div
		class={`flex flex-row gap-2 items-center transition mr-2 rounded-md ${stateSelected ? 'preset-filled-primary-500' : 'preset-filled-surface-50-950 hover:preset-filled-primary-100-900'} p-2`}
	>
		<IconRender size={18} />
		<span class="text-base overflow-ellipsis max-w-[10rem]">{text}</span>
	</div>
{/snippet}

{#snippet listGroupItem(text, IconRender, onclick, stateSelected, menus)}
    <!-- svelte-ignore a11y_invalid_attribute -->
    <a href='#' {onclick}>
        <ContextMenuProvider {menus}>
			<div
				class={`flex flex-row gap-2 items-center transition mr-2 rounded-md ${stateSelected ? 'preset-filled-primary-500' : 'preset-filled-surface-50-950 hover:preset-filled-primary-100-900'} p-2`}
			>
				<IconRender size={18} />
				<span class="text-base overflow-ellipsis max-w-[10rem]">{text}</span>
			</div>
		</ContextMenuProvider>
	</a>
{/snippet}

<div
	use:disableContextMenu
	class="preset-filled-surface-50 w-[15rem] flex h-full flex-col overflow-scroll-clip"
>
	{@render listSection($_('common.product_name'))}
	{@render listItem($_('main.menu.create_feeds_package'), IconAddFeed, onAddFeedPressed, false, null)}
	{@render listItem($_('main.menu.settings'), IconSettings, openSettings, false, null)}

	{@render listSection($_('main.section_frequently_used.label'))}
	{@render listItem($_('main.section_frequently_used.menu.today'), IconToday, onSelectToday, isTodaySelected, null)}
	{@render listItem($_('main.section_frequently_used.menu.this_week'), IconWeekend, onSelectWeekend, isWeekendSelected, null)}
	{@render listItem($_('main.section_frequently_used.menu.favorites'), IconFavorites, onSelectFavorite, isFavoriteSelected, null)}
	{@render listItem($_('main.section_frequently_used.menu.unread'), IconUnread, onSelectUnread, isUnreadSelected, null)}

	{@render listSectionWithAction($_('main.section_subscriptions.label'), IconAdd, onAddFeedPressed)}

	<div class="flex flex-col h-full overflow-y-auto overflow-x-clip">
		{#if store != null && store.loadingStore.status === Status.Completed}
			{#each store.feedPackages as feedPackage (feedPackage.id)}
				{@render listGroupItem(
					feedPackage.name,
					expandState[feedPackage.id] ? IconPackageExpand : IconPackageUnExpand,
					() => toggleExpand(feedPackage.id),
					false,
					createFeedPackageMenus(feedPackage)
				)}
				{#if expandState[feedPackage.id]}
					{#each feedPackage.feeds as feed (feed.id)}
						<div class="pl-6">
							{@render listItem(
								feed.name,
								IconScrapSource,
								() => onFeedPressed(feed.id),
								selectedFeedId === feed.id,
								createFeedMenus(feedPackage, feed)
							)}
						</div>
					{/each}
				{/if}
			{/each}
		{/if}
	</div>
</div>
