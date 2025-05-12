<script>
	import { _ } from 'svelte-i18n';
	import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
	import { featuresApi } from '$lib/hybrid-apis/feed/impl';
	import { browser } from '$app/environment';
	import { disableContextMenu } from '$lib/utils/dom';

	let appName = $state('');
	let appVersion = $state('-.-.-');
	let engineVersion = $state('-.-.-');
	function onVisitHome() {
		featuresApi.open_article_external('https://aiqino.netlify.app?s=desktop');
	}
	if (browser) {
		getName().then((val) => (appName = val));
		getVersion().then((val) => (appVersion = val));
		getTauriVersion().then((val) => (engineVersion = val));
	}
</script>

<div
	use:disableContextMenu
	class="flex flex-col items-center content-center justify-center bg-surface-50 p-6 h-screen"
>
	<div class="flex flex-col items-center">
		<img class="w-32 h-32 object-contain" src="./favicon.png" alt="logo" />
		<p class="mt-4 h5 text-surface-900 min-h-7">{appName}</p>
		<p class="text-surface-500 text-xs">{`${$_('about.ver_app')}：${appVersion}`}</p>
		<p class="text-surface-500 text-xs">{`${$_('about.ver_engine')}：${engineVersion}`}</p>
		<button
			type="button"
			class="mt-2 btn min-w-40 preset-filled-primary-500 text-sm"
			onclick={onVisitHome}>{$_('about.visit_home')}</button
		>
		<div class="h-6"></div>
	</div>
</div>
