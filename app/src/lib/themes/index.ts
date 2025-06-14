import { setTheme as setAppTheme } from '@tauri-apps/api/app';
// function setAppTheme() {}

type ThemePresets = 'light' | 'dark';
const LITE_STORAGE_KEY_THEME = 'ls_user_theme';

function getTheme(): ThemePresets {
	return (window.localStorage.getItem(LITE_STORAGE_KEY_THEME) || 'light') as ThemePresets;
}

function setTheme(theme: ThemePresets) {
	setAppTheme(theme);
	window.localStorage.setItem(LITE_STORAGE_KEY_THEME, theme);
}

function applyTheme(): ThemePresets {
	const theme = getTheme();
	setAppTheme(theme);
	setWebInnerOnly(theme);
	return theme;
}

function setWebInnerOnly(theme: ThemePresets) {
	document.documentElement.classList.toggle('dark', theme === 'dark');
}

export { applyTheme, getTheme, setTheme, setWebInnerOnly };
export type { ThemePresets };
