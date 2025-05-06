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

<div class="p-4 pt-0 select-auto w-full wrapper">
    <SvelteMarkdown {source} {renderers} />
</div>

<style>
	.wrapper :global(h1) {
		font-size: 1.7rem;
		font-weight: bold;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(h2) {
		font-size: 1.5rem;
		font-weight: bold;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(h3) {
	    font-size: 1.3rem;
		font-weight: bold;
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(p) {
	   margin-top: 0.5rem;
	}
	.wrapper :global(h4) {
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	.wrapper :global(strong) {
		margin-top: 1.5rem;
		margin-bottom: 0.5rem;
	}
	/* .wrapper :global(p) {
		text-indent: 2em;
	} */
	.wrapper :global(ol) {
    	margin-top: 0.5rem;
    	margin-bottom: 1.5rem;
    	margin-left: 1.5rem;
        list-style-type: decimal;
	}
	.wrapper :global(ul) {
	    margin-top: 0.5rem;
        margin-bottom: 1.5rem;
        margin-left: 0rem;
	    list-style-type: disc;
	}
	.wrapper :global(ul li) {
		list-style-type: disc; /* 设置列表项前的标记为圆点 */
		margin-top: 0.25rem;
		margin-left: 0.25rem;
		margin-bottom: 0.25rem;
	}
	.wrapper :global(ul li::before) {
        content: "•"; /* 要插入的特定字符 */
        margin-right: 5px; /* 字符与文本之间的间距 */
    }
    .wrapper :global(table) {
        margin-top: 1rem;
    }
    .wrapper :global(th) {
        border: 1px solid rgb(160 160 160);
        padding: 8px 10px;
    }
    .wrapper :global(td) {
        border: 1px solid rgb(160 160 160);
        padding: 8px 10px;
    }
    .wrapper :global(code) {
        max-width: 800px;
        overflow-x: auto;
        display: block;
    }
</style>
