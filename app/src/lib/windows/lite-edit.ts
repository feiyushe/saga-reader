import type { UnlistenFn } from '@tauri-apps/api/event';
import { openWithCallback } from './utils';
import type { FeedsPackage, FeedTargetDescription } from '$lib/hybrid-apis/feed/types';
import type { WindowOptions } from '@tauri-apps/api/window';

function openFeedPackageCreateWindow(): Promise<FeedsPackage> {
	return new Promise((resolve) => {
		open(
			'window_feed_create',
			'新增订阅组',
			`/feedsPackage/create_or_edit?mode=create`,
			(submited: boolean, data: string) => {
				const result = JSON.parse(data) as FeedsPackage;
				resolve(result);
			},
			{ height: 220 }
		);
	});
}

function openFeedPackageEditWindow(
	id: string,
	name: string
): Promise<{ submited: boolean; newName: string }> {
	return new Promise((resolve) => {
		open(
			'window_feeds_package_edit',
			'编辑订阅组',
			`/feedsPackage/create_or_edit?mode=edit&id=${id}&name=${name}`,
			(submited: boolean, newName: string) => {
				resolve({ submited, newName });
			},
			{ height: 220 }
		);
	});
}

function openFeedCreateWindow(feedsPackageId: string): Promise<FeedTargetDescription> {
	return new Promise((resolve) => {
		open(
			'window_feeds_package_create',
			'新增订阅',
			`/feeds/create_or_edit?mode=create&feedsPackageId=${feedsPackageId}`,
			(submited: boolean, data: string) => {
				const result = JSON.parse(data) as FeedTargetDescription;
				resolve(result);
			},
			{ height: 400 }
		);
	});
}

function openFeedEditWindow(
	id: string,
	name: string,
	fetcher_id: string,
	data: string[],
	feedsPackageId: string
): Promise<{ submited: boolean; newName: string }> {
	return new Promise((resolve) => {
		open(
			'window_feed_edit',
			'编辑订阅',
			`/feeds/create_or_edit?mode=edit&id=${id}&name=${name}&fetcher_id=${fetcher_id}&data=${JSON.stringify(data)}&feedsPackageId=${feedsPackageId}`,
			(submited: boolean, newName: string) => {
				resolve({ submited, newName });
			},
			{ height: 400 }
		);
	});
}

async function open(
	label: string,
	title: string,
	url: string,
	onFinish: (submited: boolean, data: string) => void,
	windowOpt: WindowOptions = {}
) {
	let disposer: UnlistenFn | null = null;
	const onCloseWithAutoDispose = (data: string) => {
		console.log('onCloseWithAutoDispose...callback', data);
		onFinish(!!data, data);
		if (disposer) disposer();
	};
	disposer = await openWithCallback(label, url, onCloseWithAutoDispose, {
		title,
		width: 600,
		center: true,
		resizable: false,
		maximizable: false,
		alwaysOnTop: true,
		...windowOpt
	});
}

export {
	open,
	openFeedPackageCreateWindow,
	openFeedPackageEditWindow,
	openFeedCreateWindow,
	openFeedEditWindow
};
