import { showWindowSingleton } from './utils';

async function open() {
	showWindowSingleton('settings', '/settings', {
		title: '应用设置',
		width: 600,
		height: screen.availHeight - 50,
		center: true,
		resizable: false,
		maximizable: false
	});
}

export { open };
