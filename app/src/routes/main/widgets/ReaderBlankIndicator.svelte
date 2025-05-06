<script lang="ts">
  import { _ } from 'svelte-i18n';
</script>

<div class="w-full h-full flex items-center justify-center">
	<div class="cursor-default w-min-96 flex flex-col items-start text-surface-500">
		<h6 class="h6 mb-1">{$_('main.reader.blank_tip_0')}</h6>
		<span class="mb-1">{$_('main.reader.blank_tip_1')}</span>
		<span class="mb-1">{$_('main.reader.blank_tip_2')}</span>
		<span class="mb-1">{$_('main.reader.blank_tip_3')}</span>
	</div>
</div>
