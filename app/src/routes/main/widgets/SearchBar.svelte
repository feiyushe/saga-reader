<script lang="ts">
    import { _ } from 'svelte-i18n';
	import type { SearchBarProps } from './types';
	import IconRefresh from 'lucide-svelte/icons/refresh-cw';

	const { store, articles_store }: SearchBarProps = $props();
</script>

<div class="mb-2 mt-2 flex flex-row items-center min-h-8">
	<input
		type="search"
		bind:value={store.filterText}
		class="min-h-[auto] w-full rounded border-0 px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear preset-filled-surface-50-950 focus:placeholder:opacity-100 motion-reduce:transition-none"
		placeholder={$_('main.search.placeholder_input')}
		id="searchControlInput"
	/>
	{#if !articles_store.isFeedSpecified}
		<button
			type="button"
			class="ml-2 h-full btn preset-filled-primary-500"
			onclick={articles_store.updateFeeds}
		>
			<IconRefresh size={18} />
		</button>
	{/if}
</div>
