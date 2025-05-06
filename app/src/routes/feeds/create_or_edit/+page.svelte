<script lang="ts">
    import { _ } from 'svelte-i18n';
	import { browser } from '$app/environment';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import SaveOperatePanel from '$lib/widgets/SaveOperatePanel.svelte';
	import { format, isTextEmpty } from '$lib/utils/text';
	import { featuresApi } from '$lib/hybrid-apis/feed/impl';
	import { genStringId } from '$lib/utils/id';
	import type { FeedFetcherID } from '$lib/hybrid-apis/feed/types';
	import { disableContextMenu } from '$lib/utils/dom';

	type Mode = 'create' | 'edit';

	let loaded = $state(false);
	let mode: Mode = $state('create');
	let callbackEventId: string = $state('');
	let formId = $state('');
	let formName = $state('');
	let formFetcherId: FeedFetcherID = $state('scrap');
	let formData: string[] = [];
	let formDataString = $state('');
	let formFeedPackagesId = $state('');
	let submitErrorMessage: string | null = $state(null);

	const formErrorName = $derived(isTextEmpty(formName));
	const formErrorFetcherId = $derived(isTextEmpty(formFetcherId));
	const formErrorData = $derived(isTextEmpty(formDataString));
	const canSave = $derived(!formErrorName && !formErrorData && !formErrorFetcherId);
	const formLabelData = $derived.by(() => {
		if (formFetcherId === 'scrap') return $_('main.feed.create_or_edit.field_keywords');
		if (formFetcherId === 'rss') return $_('main.feed.create_or_edit.field_rss_url');
		return format($_('tip_unknown_fetcher'), { formFetcherId });
	});
	const formInputPlaceHolderData = $derived.by(() => {
		if (formFetcherId === 'scrap')
			return $_('main.feed.create_or_edit.field_keywords_placeholder');
		if (formFetcherId === 'rss') return $_('main.feed.create_or_edit.field_rss_url_placeholder');
		return '';
	});

	if (browser) {
		const urlParams = new URLSearchParams(window.location.search);
		mode = (urlParams.get('mode') || 'create') as Mode;
		formId = decodeURIComponent(urlParams.get('id') || '');
		formFeedPackagesId = decodeURIComponent(urlParams.get('feedsPackageId') || '');
		formName = decodeURIComponent(urlParams.get('name') || '');
		formFetcherId = decodeURIComponent(urlParams.get('fetcher_id') || 'scrap') as FeedFetcherID;
		formData = JSON.parse(decodeURIComponent(urlParams.get('data') || '[]'));
		formDataString = formData.join(' ');
		callbackEventId = urlParams.get('callbackEventId') || '';
		loaded = true;
	}

	async function onSave() {
		formData = formDataString.split(' ');
		if (mode === 'create') {
			featuresApi.add_feed(formFeedPackagesId, {
				id: genStringId(),
				name: formName,
				fetcher_id: formFetcherId,
				data: formData
			});
		} else if (mode === 'edit') {
			try {
				await featuresApi.rename_feed(formFeedPackagesId, formId, formName);
				await featuresApi.change_feed_data(formFeedPackagesId, formId, formData);
			} catch (e) {
				submitErrorMessage = format($_('main.feed.create_or_edit.tip_modify_failured'), { e: 'features api execute failure' });
				console.error(e);
				throw e;
			}
		} else {
			throw Error(`feed create_or_edit.onSave error, unknown mode = ${mode}`);
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
	<div use:disableContextMenu class="cursor-default p-4">
		<label class="label mt-2">
			<span class="label-text type-scale-2">{$_('main.feed.create_or_edit.field_feed_name')}</span>
			<div class="h-1"></div>
			<!-- svelte-ignore a11y_autofocus -->
			<input
				class="input p-2"
				maxlength="10"
				autofocus
				type="text"
				bind:value={formName}
				placeholder={$_('main.feed.create_or_edit.field_feed_placeholder')}
			/>
		</label>

		<label class="label mt-4">
			<span class="label-text type-scale-2">{$_('main.feed.create_or_edit.field_fetcher_type_name')}</span>
			<div class="h-1"></div>
			<select class="select cursor-pointer" disabled={mode === 'edit'} bind:value={formFetcherId}>
				<option value="scrap">{$_('main.feed.create_or_edit.field_fetcher_type_selection_1')}</option>
				<option value="rss">{$_('main.feed.create_or_edit.field_fetcher_type_selection_2')}</option>
			</select>
		</label>

		<label class="label mt-4">
			<span class="label-text type-scale-2">{formLabelData}</span>
			<div class="h-1"></div>
			<input
				class="input p-2"
				type="text"
				bind:value={formDataString}
				placeholder={formInputPlaceHolderData}
			/>
		</label>

		{#if submitErrorMessage}
			<p class="mt-2 text-error-500 type-scale-1">
				{submitErrorMessage}
			</p>
		{:else}
			<p class="mt-2 text-surface-400 type-scale-1">
			     {$_('main.feed.create_or_edit.tip_footer')}
			</p>
		{/if}

		<SaveOperatePanel {canSave} {onSave} {onCancel} />
	</div>
{/if}
