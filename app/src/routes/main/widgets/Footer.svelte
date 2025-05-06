<script lang="ts">
    import { _ } from 'svelte-i18n';
	import { disableContextMenu } from '$lib/utils/dom';
	import { ProgressRing } from '@skeletonlabs/skeleton-svelte';
	import IconX from 'lucide-svelte/icons/x';
	import { Popover } from '@skeletonlabs/skeleton-svelte';
	import { Status } from '../stores//loading.svelte';
	import type { FooterProps } from './types';
	const { tasksStore }: FooterProps = $props();

	let openState = $state(false);

	function popoverClose() {
		openState = false;
	}
</script>

<footer use:disableContextMenu class="flex flex-row flex-0 min-h-6 bg-blue-500 text-sm">
	<!-- left area -->
	<span class="flex-0 min-w-32"></span>
	<!-- center area -->
	<div class="flex-1"></div>

	<Popover
		open={openState}
		onOpenChange={(e) => (openState = e.open)}
		positioning={{ placement: 'top' }}
		triggerBase="p-0"
		contentBase="card shadow-[0px_0px_10px_1px_rgba(0,0,0,0.3)] preset-filled-surface-50-950 p-4 min-w-[320px]"
		arrow
		arrowBackground="!bg-surface-50 dark:!bg-surface-950"
	>
		{#snippet trigger()}
			<div
				class={`flex-0 ${openState ? 'bg-primary-600' : ''} hover:bg-primary-600 flex flex-row min-w-64 items-center pl-2 min-h-6 pr-2 text-surface-50`}
			>
				{#if tasksStore.pendingStatus === Status.Loading}
					<ProgressRing
						classes="mr-2"
						value={null}
						size="size-4"
						strokeWidth="2px"
						meterStroke="stroke-primary-500"
						trackStroke="stroke-primary-100"
					/>
				{/if}

				<span>{tasksStore.pendingStatusText}</span>
			</div>
		{/snippet}
		{#snippet content()}
			<header class="flex justify-between">
				<p class="cursor-default font-bold mt-1.5 type-scale-2">{$_('main.footer.tasks.label')}</p>
				<button class="btn-icon hover:preset-tonal" onclick={popoverClose}><IconX /></button>
				<!-- svelte-ignore a11y_autofocus -->
				<button autofocus aria-label="hidden_close" class="hidden"></button>
			</header>
			<article class="cursor-default">
				{#if tasksStore.pendings?.length > 0}
					<div class="h-2"></div>
					{#each tasksStore.pendings as pending (pending.description)}
						<div class="flex flex-row pb-2 type-scale-1 items-center">
							<span class="badge preset-filled-primary-500">{pending.loadingStore.statusText}</span>
							<div class="w-2"></div>
							{pending.description}
						</div>
					{/each}
				{:else}
					<span class="text-surface-500 type-scale-1">{$_('main.footer.tasks.idle')}</span>
				{/if}
			</article>
		{/snippet}
	</Popover>
</footer>
