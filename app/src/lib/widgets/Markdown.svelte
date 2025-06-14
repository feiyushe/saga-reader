<script lang="ts">
	import SvelteMarkdown from '@humanspeak/svelte-markdown';
	import type { MarkdownProps } from './types';
	import type { Component } from 'svelte';
	import MarkdownImg from './MarkdownImg.svelte';

	interface HtmlRenderers {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		[key: string]: Component<any, any, any> | null;
	}

	const { value: source }: MarkdownProps = $props();
	const renderers: Partial<HtmlRenderers> = {
		image: MarkdownImg
	};
</script>

<div class="p-4 pt-0 select-auto w-full wrapper text-surface-800-200">
	<SvelteMarkdown {source} {renderers} />
</div>

<style>
	.wrapper :global(h1) {
		font-size: 1.5rem;
		font-weight: bold;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		line-height: 1.75;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(h2) {
		font-size: 1.3rem;
		font-weight: bold;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		line-height: 1.75;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(h3) {
		font-size: 1.2rem;
		font-weight: bold;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 16px;
		line-height: 1.75;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(p) {
		margin-top: 0.5rem;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(h4) {
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1.1rem;
		line-height: 1.75;
	}
	.wrapper :global(strong) {
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	/* .wrapper :global(p) {
		text-indent: 2em;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 16px;
		line-height: 1.75;
		color: #3c4043;
	} */
	.wrapper :global(ol) {
		margin-top: 0.5rem;
		margin-bottom: 1.5rem;
		margin-left: 1.5rem;
		list-style-type: decimal;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(ul) {
		margin-top: 0.5rem;
		margin-bottom: 1.5rem;
		margin-left: 0rem;
		list-style-type: disc;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(ul li) {
		list-style-type: disc; /* 设置列表项前的标记为圆点 */
		margin-top: 0.25rem;
		margin-left: 0.25rem;
		margin-bottom: 0.25rem;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(ul li::before) {
		content: '•'; /* 要插入的特定字符 */
		margin-right: 5px; /* 字符与文本之间的间距 */
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(table) {
		margin-top: 1rem;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(th) {
		border: 1px solid rgb(160 160 160);
		padding: 8px 10px;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(td) {
		border: 1px solid rgb(160 160 160);
		padding: 8px 10px;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
	.wrapper :global(code) {
		max-width: 800px;
		overflow-x: auto;
		font-family: 'Google Sans', Roboto, Arial, sans-serif;
		font-size: 1rem;
		line-height: 1.75;
	}
</style>
