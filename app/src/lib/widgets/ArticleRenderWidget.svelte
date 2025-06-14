<script lang="ts">
	/** eslint-disable svelte/no-at-html-tags */
	import type { ArticleRenderProps, ArticleRenderType } from './types';
	import Markdown from './Markdown.svelte';
	import { removeCodeBlockWrapper } from '$lib/utils/text';
	import { featuresApi } from '$lib/hybrid-apis/feed/impl';
	import { onMount } from 'svelte';

	const { value }: ArticleRenderProps = $props();
	const purgedHtml = $derived(removeCodeBlockWrapper(value));
	const renderType: ArticleRenderType = $derived(purgedHtml[0] === '<' ? 'html' : 'markdown');
	let htmlContainer: HTMLDivElement | null = $state(null);

	onMount(() => {
		if (!htmlContainer) return;
		const anchorClickInterceptor = (event: MouseEvent) => {
			// 检查点击的元素是否是一个链接
			const target = event.target as HTMLElement;
			if (target?.tagName === 'A') {
				// 阻止默认的链接跳转行为
				event.preventDefault();
				// 获取链接的 href 属性
				const url = (target as HTMLAnchorElement).href;
				// 调用特定函数来处理链接
				featuresApi.open_article_external(url);
			}
		};
		(htmlContainer as HTMLDivElement).addEventListener('click', anchorClickInterceptor);
		return () => {
			(htmlContainer as HTMLDivElement).removeEventListener('click', anchorClickInterceptor);
		};
	});
</script>

{#if renderType === 'html'}
	<div bind:this={htmlContainer} class="p-6 preset-filled-surface-50-950">{@html purgedHtml}</div>
{:else}
	<Markdown {value} />
{/if}
