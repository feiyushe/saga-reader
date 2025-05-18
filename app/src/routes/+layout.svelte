<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { applyTheme, setWebInnerOnly } from '$lib/themes';

	onMount(() => {
		const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

		const themeListener = (event: MediaQueryListEvent) => {
			setWebInnerOnly(event.matches ? 'dark' : 'light');
		};
		darkModeMediaQuery.addEventListener('change', themeListener);

		applyTheme();

		return () => darkModeMediaQuery.removeEventListener('change', themeListener);
	});
</script>

<slot></slot>
