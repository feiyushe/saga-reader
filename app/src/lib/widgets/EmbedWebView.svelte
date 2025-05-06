<script lang="ts">
	import { observeVisiblity } from '$lib/utils/dom';
	import { LoadingStatus } from '$lib/types/loading';
	import type { EmbedWebViewProps } from './types';
	import { Progress } from '@skeletonlabs/skeleton-svelte';

	let { src, onLoadStart, onLoadEnd }: EmbedWebViewProps = $props();
	let loadingStatus: LoadingStatus = $state(LoadingStatus.Loading);
	let webview: HTMLIFrameElement | null = $state(null);

	let hasRequestToUse = $state(false);

	const onLoadEventHandler = () => {
		loadingStatus = LoadingStatus.Completed;
		console.log('onLoadEventHandler...completed', src, loadingStatus);
		onLoadEnd?.(src);
	};
	$effect.pre(() => {
		const snapshotSrc = src;
		loadingStatus = LoadingStatus.Loading;
		onLoadStart?.(snapshotSrc /* important, 去掉snapshot只传src不会触发更新 */);

		webview?.addEventListener('load', onLoadEventHandler);
		return () => webview?.removeEventListener('load', onLoadEventHandler);
	});

	function onVisiblityChanged(v: boolean) {
		hasRequestToUse = v;
	}
</script>

<div
	use:observeVisiblity={{ callback: onVisiblityChanged }}
	class="flex h-full w-full flex-col items-center"
>
	{#if hasRequestToUse}
		{#if loadingStatus === LoadingStatus.Loading}
			<Progress
				classes="mt-12"
				width="w-96"
				value={null}
				meterAnimate="embed_web_loading_indicator"
				meterBg="bg-primary-500"
			/>
		{/if}
		<iframe
			bind:this={webview}
			sandbox="allow-scripts allow-forms allow-same-origin"
			class={`h-full w-full ${loadingStatus === LoadingStatus.Completed ? 'visible' : 'hidden'}`}
			title="Embedded Web Page"
			{src}
		></iframe>
	{/if}
</div>

<style>
	:global(.embed_web_loading_indicator) {
		animation: my-custom-animation 2s ease-in-out infinite;
	}
	@keyframes my-custom-animation {
		0% {
			translate: -100%;
		}
		25% {
			scale: 1;
		}
		50% {
			scale: 0.25 1;
			translate: 50%;
		}
		75% {
			scale: 1;
		}
		100% {
			translate: 200%;
		}
	}
</style>
