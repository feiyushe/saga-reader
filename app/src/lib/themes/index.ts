import { setTheme as setAppTheme } from '@tauri-apps/api/app';

type ThemePresets = 'light' | 'dark';
const LITE_STORAGE_KEY_THEME = 'ls_user_theme';

function getTheme(): ThemePresets {
	return (window.localStorage.getItem(LITE_STORAGE_KEY_THEME) || 'light') as ThemePresets;
}

function setTheme(theme: ThemePresets) {
	setAppTheme(theme);
	window.localStorage.setItem(LITE_STORAGE_KEY_THEME, theme);
}

function applyTheme() {
	const theme = getTheme();
	setAppTheme(theme);
}

export { applyTheme, getTheme, setTheme };
export type { ThemePresets };
