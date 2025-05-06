import { featuresApi } from '$lib/hybrid-apis/feed/impl';
import type { FeedsPackage, FeedTargetDescription } from '$lib/hybrid-apis/feed/types';
import {
	Status,
	create as createLoadingStore,
	type StoreType as LoadingStore
} from './loading.svelte';

type StoreType = {
	loadingStore: LoadingStore;
	feedPackages: FeedsPackage[];
	refresh: () => Promise<void>;
	addFeedsPackage: (feedsPackage: FeedsPackage) => Promise<void>;
	removeFeedsPackage: (packageId: string) => Promise<void>;
	renameFeedsPackage: (packageId: string, newName: string) => Promise<void>;
	addFeed: (packageId: string, ftd: FeedTargetDescription) => Promise<void>;
	removeFeed: (packageId: string, feedId: string) => Promise<void>;
	renameFeed: (packageId: string, feedId: string, newName: string) => Promise<void>;
	findPackagesOwnerByFeedId: (feedId: string) => FeedsPackage | undefined;
};

function create(): StoreType {
	const loadingStore = createLoadingStore(Status.Loading);
	let feedPackages: FeedsPackage[] = $state([]);

	async function refresh() {
		return featuresApi
			.get_feeds_packages()
			.then((data) => {
				feedPackages = data;
				loadingStore.complete();
			})
			.catch((e) => {
				loadingStore.error(e);
			});
	}

	function findPackagesOwnerByFeedId(feedId: string): FeedsPackage | undefined {
		return feedPackages.find((feedPackage) => {
			return feedPackage.feeds.findIndex((feed) => feed.id === feedId) >= 0;
		});
	}

	function addFeedsPackage(feedsPackage: FeedsPackage): Promise<void> {
		return featuresApi.add_feeds_package(feedsPackage);
	}

	function removeFeedsPackage(packageId: string): Promise<void> {
		return featuresApi.remove_feeds_package(packageId);
	}

	function renameFeedsPackage(packageId: string, newName: string): Promise<void> {
		return featuresApi.rename_feeds_package(packageId, newName);
	}

	function addFeed(packageId: string, ftd: FeedTargetDescription): Promise<void> {
		return featuresApi.add_feed(packageId, ftd);
	}

	function removeFeed(packageId: string, feedId: string): Promise<void> {
		return featuresApi.remove_feed(packageId, feedId);
	}

	function renameFeed(packageId: string, feedId: string, newName: string): Promise<void> {
		return featuresApi.rename_feed(packageId, feedId, newName);
	}

	return {
		loadingStore,
		get feedPackages() {
			return feedPackages;
		},
		refresh,
		addFeedsPackage,
		removeFeedsPackage,
		renameFeedsPackage,
		addFeed,
		removeFeed,
		renameFeed,
		findPackagesOwnerByFeedId
	};
}

export type { StoreType };

export { create };
