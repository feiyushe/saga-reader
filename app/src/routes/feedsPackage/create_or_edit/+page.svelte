<script lang="ts">
    import { _ } from 'svelte-i18n';
	import { browser } from '$app/environment';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import SaveOperatePanel from '$lib/widgets/SaveOperatePanel.svelte';
	import { format, isTextEmpty } from '$lib/utils/text';
	import { featuresApi } from '$lib/hybrid-apis/feed/impl';
	import { genStringId } from '$lib/utils/id';
	import { disableContextMenu } from '$lib/utils/dom';

	type Mode = 'create' | 'edit';

	let loaded = $state(false);
	let mode: Mode = $state('create');
	let callbackEventId: string = $state('');
	let formId = $state('');
	let formName = $state('');
	let submitErrorMessage: string | null = $state(null);

	const formErrorName = $derived(isTextEmpty(formName));
	const canSave = $derived(!formErrorName);

	if (browser) {
		const urlParams = new URLSearchParams(window.location.search);
		mode = (urlParams.get('mode') || 'create') as Mode;
		formId = decodeURIComponent(urlParams.get('id') || '');
		formName = decodeURIComponent(urlParams.get('name') || '');
		callbackEventId = decodeURIComponent(urlParams.get('callbackEventId') || '');
		loaded = true;
	}

	async function onSave() {
		if (mode === 'create') {
			featuresApi.add_feeds_package({
				id: genStringId(),
				name: formName,
				feeds: [],
				is_flat_on_root: false
			});
		} else if (mode === 'edit') {
			try {
				await featuresApi.rename_feeds_package(formId, formName);
			} catch (e) {
				submitErrorMessage = format($_('main.feeds.create_or_edit.tip_footer'), { e: "the features api `rename_feeds_package` execute failure"});
				throw e;
			}
		} else {
			throw Error(`feedspackage create_or_edit.onSave error, unknown mode = ${mode}`);
		}

		const window = getCurrentWindow();
		window.emit(callbackEventId, JSON.stringify({}));
		window.close();
	}

	function onCancel() {
		const window = getCurrentWindow();
		window.emit(callbackEventId);
		window.close();
	}
</script>

{#if loaded}
	<div use:disableContextMenu class="p-4 cursor-default">
		<label class="label mt-2">
			<span class="label-text type-scale-2">{$_('main.feeds.create_or_edit.field_feeds_package_name')}</span>
			<div class="h-1"></div>
			<!-- svelte-ignore a11y_autofocus -->
			<input
				class="input p-2"
				autofocus
				maxlength="10"
				type="text"
				bind:value={formName}
				placeholder={$_('main.feeds.create_or_edit.field_feeds_package_placeholder')}
			/>
		</label>
		{#if submitErrorMessage}
			<p class="mt-2 text-error-500 type-scale-1">
				{submitErrorMessage}
			</p>
		{:else}
			<p class="mt-2 text-surface-400 type-scale-1">
				{$_('main.feeds.create_or_edit.tip_footer')}
			</p>
		{/if}

		<SaveOperatePanel {canSave} {onSave} {onCancel} />
	</div>
{/if}
