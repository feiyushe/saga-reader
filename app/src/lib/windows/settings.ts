import { showWindowSingleton } from './utils';

async function open() {
	showWindowSingleton('settings', '/settings', {
		title: '麒睿智库',
		width: 600,
		height: 750,
		center: true,
		resizable: false,
		maximizable: false
	});
}

export { open };
