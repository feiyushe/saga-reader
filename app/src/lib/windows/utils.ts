import { Window, getAllWindows, type WindowOptions } from '@tauri-apps/api/window';
import { Webview } from '@tauri-apps/api/webview';
import type { UnlistenFn } from '@tauri-apps/api/event';

async function openWithCallback(
	label: string,
	url: string,
	onClose: (data: string) => void,
	windowOpt?: WindowOptions
): Promise<UnlistenFn> {
	const eventId = `event-openWithCallback-${Date.now()}`;
	const urlWithEventId =
		url.indexOf('?') < 0
			? `${url}?callbackEventId=${eventId}`
			: `${url}&callbackEventId=${eventId}`;
	const window = await showWindowSingleton(label, urlWithEventId, windowOpt);
	return window.listen<string>(eventId, (e) => onClose(e.payload));
}

async function showWindowSingleton(
	label: string,
	url: string,
	windowOpt?: WindowOptions
): Promise<Window> {
	const windows = await getAllWindows();
	const existed_window = windows.find((w) => w.label === label);
	if (existed_window) {
		existed_window.setFocus();
		return existed_window;
	}

	const new_window = new Window(label, windowOpt);
	new Webview(new_window, label, {
		url: url,
		x: 0,
		y: 0,
		width: windowOpt?.width || 100,
		height: windowOpt?.height || 100
	});
	return new_window;
}

export { showWindowSingleton, openWithCallback };
