<script lang="ts">
    import { _ } from 'svelte-i18n';
	import type { MouseEventHandler } from 'svelte/elements';

	const {
		onSave,
		onCancel,
		canSave
	}: {
		onSave: MouseEventHandler<HTMLButtonElement>;
		onCancel: MouseEventHandler<HTMLButtonElement>;
		canSave: boolean;
	} = $props();
</script>

<div class="pt-4 flex justify-end space-x-2">
	<button type="button" disabled={!canSave} class="btn preset-filled-primary-500" onclick={onSave}
		>{$_('common_dialog.save')}</button
	>
	<button type="button" class="btn preset-tonal-surface" onclick={onCancel}>{$_('common_dialog.cancel')}</button>
</div>
