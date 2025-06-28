<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { AISpriteProps } from '../widgets/types';
	import { Avatar, ProgressRing } from '@skeletonlabs/skeleton-svelte';
	import { Bot, Baby as UserIcon, SendHorizontal, XIcon } from 'lucide-svelte';
	import Markdown from '$lib/widgets/Markdown.svelte';
	import { spriteToaster as toaster } from '../stores/toast';

	import { onMount } from 'svelte';
	let { store }: AISpriteProps = $props();

	let elemChat: HTMLElement | undefined = $state();
	let currentMessage = $state('');

	onMount(() => {
		scrollChatBottom();
	});

	function scrollChatBottom(behavior?: 'auto' | 'instant' | 'smooth') {
		elemChat?.scrollTo({ top: elemChat?.scrollHeight, behavior });
	}

	async function addMessage() {
		if (store.isLoading) {
			toaster.info({
				description: $_('aisprite.tip_wait_llm_response')
			});
			return;
		}

		if (!currentMessage) {
			toaster.info({
				description: $_('aisprite.tip_no_input')
			});
			return;
		}

		const payload = currentMessage;
		currentMessage = '';
		requestAnimationFrame(() => {
			scrollChatBottom('smooth');
		});
		const success = await store.send({
			mtype: 'text',
			payload
		});
		if (!success) {
			toaster.info({
				description: $_('aisprite.tip_error_llm_error')
			});
		}
		requestAnimationFrame(() => {
			scrollChatBottom('smooth');
		});
	}

	function onPromptKeydown(event: KeyboardEvent) {
		if (event.keyCode === 13) {
			event.preventDefault();
			addMessage();
		}
	}

	function toDateText(timestamp: string) {
		return new Date(parseInt(timestamp)).toLocaleString();
	}
</script>

{#if store.opened}
	<div
		class="absolute flex flex-col overflow-scroll h-full w-[36rem] preset-filled-surface-100-900 shadow-[0px_0px_10px_1px_rgba(0,0,0,0.3)]"
	>
		<header
			class="flex justify-between preset-filled-surface-50-950 p-3 shadow-[0px_0px_10px_1px_rgba(0,0,0,0.3)]"
		>
			<p class="h6 cursor-default mt-0.5">{$_('aisprite.label')}</p>
			<button class="btn-icon hover:preset-tonal" onclick={store.toggle}><XIcon /></button>
			<!-- svelte-ignore a11y_autofocus -->
			<button autofocus aria-label="hidden_close" class="hidden"></button>
		</header>

		<!-- Conversation -->
		<section bind:this={elemChat} class="h-full p-4 overflow-y-auto space-y-4">
			{#each store.history as bubble (bubble.created_at)}
				{#if bubble.role !== 'user'}
					<div class="grid grid-cols-[auto_1fr] gap-2">
						<Avatar name={bubble.role} size="size-12" classes="preset-filled-primary-900-100">
							<Bot size="28" />
						</Avatar>
						<div
							class="card pt-4 pl-4 pr-4 pb-2 preset-tonal rounded-tl-none rounded-bl-none space-y-2"
						>
							<header class="flex justify-between items-center">
								<p class="font-bold">Qino</p>
								<small class="opacity-50">{toDateText(bubble.created_at).toLocaleString()}</small>
							</header>
							<Markdown value={bubble.payload} />
						</div>
					</div>
				{:else}
					<div class="grid grid-cols-[1fr_auto] gap-2">
						<div
							class="card pt-4 pl-4 pr-4 pb-2 preset-filled-primary-500 rounded-tr-none rounded-br-none space-y-2"
						>
							<header class="flex justify-between items-center">
								<p class="font-bold">{$_('aisprite.chat_me')}</p>
								<small class="opacity-50">{toDateText(bubble.created_at).toLocaleString()}</small>
							</header>
							<Markdown value={bubble.payload} />
						</div>
						<Avatar name={bubble.role} size="size-12" classes="preset-filled-primary-500">
							<UserIcon size="32" />
						</Avatar>
					</div>
				{/if}
			{/each}

			{#if store.isLoading}
				<ProgressRing
					classes="mt-2"
					value={null}
					size="size-6"
					strokeWidth="5px"
					meterStroke="stroke-primary-500"
					trackStroke="stroke-primary-100"
				/>
			{/if}
		</section>
		<hr class="hr" />
		<!-- Prompt -->
		<section class="p-4 mb-12">
			<div
				class="input-group grid-cols-[auto_1fr_auto] divide-x divide-surface-200-800 rounded-container-token"
			>
				<button disabled class="pl-2 pr-2 input-group-cell preset-tonal cursor-not-allowed"
					>+</button
				>
				<!-- svelte-ignore a11y_autofocus -->
				<textarea
					autofocus
					value={currentMessage}
					oninput={(e) => (currentMessage = e.currentTarget.value)}
					class="resize-none bg-transparent border-0 ring-0"
					name="prompt"
					id="prompt"
					placeholder={$_('aisprite.tip_placeholder_please_input')}
					rows={Math.min(Math.trunc(currentMessage.length / 26) + 1, 3)}
					onkeydown={onPromptKeydown}
				></textarea>
				{#if store.isLoading}
					<button class="pl-2 pr-2 input-group-cell preset-tonal cursor-not-allowed" disabled>
						<ProgressRing
							classes="mr-2"
							value={null}
							size="size-4"
							strokeWidth="2px"
							meterStroke="stroke-primary-500"
							trackStroke="stroke-primary-100"
						/>
					</button>
				{:else}
					<button
						class="pl-2 pr-2 input-group-cell {currentMessage
							? 'preset-filled-primary-500 cursor-pointer'
							: 'preset-tonal cursor-not-allowed'}"
						disabled={!currentMessage}
						onclick={addMessage}
					>
						<SendHorizontal />
					</button>
				{/if}
			</div>
		</section>
	</div>
{/if}

<div class="absolute left-3 bottom-3">
	<button
		type="button"
		class="btn w-60 preset-filled hover:preset-filled-primary-900-100"
		onclick={store.toggle}
	>
		<Bot />
		<span>Copilot</span>
	</button>
</div>
