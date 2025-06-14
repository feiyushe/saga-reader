import type { Article } from '$lib/types/article';

import {
	featuresApi,
	isRecentFamilyFeed,
	isSpecifyFeed,
	SPECIFY_FEED_IDSET
} from '$lib/hybrid-apis/feed/impl';
import { create as createArticles } from './articles/index.svelte';
import { create as createTasks } from './tasks.svelte';
import { create as createFeeds } from './feeds.svelte';
import { create as createReader } from './reader.svelte';
import { create as createSprite } from './sprite.svelte';
import { currentDateText } from '$lib/utils/date';
import type { IContext } from './context';

let globalSharedScheduleUpdatingFuture: Promise<void> | undefined = undefined;
let has_update_feeds_on_boot = false;

function createStore() {
	let currentFeedId: string | undefined = $state(undefined);
	let currentArticle: Article | null = $state(null);

	const context: IContext = {
		get currentArticle() {
			return currentArticle!;
		},

		get currentFeedId() {
			return currentFeedId!;
		}
	};

	const tasks = createTasks();
	const articles = createArticles(tasks);
	const feeds = createFeeds();
	const reader = createReader({ tasks });
	const sprite = createSprite(context);

	const isTodaySelected = $derived(currentFeedId === SPECIFY_FEED_IDSET.TODAY_FILTER);
	const isWeekendSelected = $derived(currentFeedId === SPECIFY_FEED_IDSET.WEEKEND_FILTER);
	const isFavoriteSelected = $derived(currentFeedId === SPECIFY_FEED_IDSET.FAVORITE_FILTER);
	const isUnreadSelected = $derived(currentFeedId === SPECIFY_FEED_IDSET.UNREAD_FILTER);

	const isFeedSpecified = $derived(isSpecifyFeed(currentFeedId));

	function setCurrentArticle(value: Article) {
		currentArticle = value;
	}

	async function getCurrentArticle(): Promise<Article> {
		if (!currentArticle) {
			return Promise.reject('the current article id is null!');
		}
		return featuresApi.query_by_id(currentArticle.id);
	}

	async function scheduleUpdate() {
		if (globalSharedScheduleUpdatingFuture) return;
		let resolve: (() => void) | null = null;
		let reject: (() => void) | null = null;
		globalSharedScheduleUpdatingFuture = new Promise((r1, r2) => {
			resolve = r1;
			reject = r2;
		});
		let haveTaskCompleted = false;
		const { list } = articles;
		for (const feedPackage of feeds.feedPackages) {
			for (const feed of feedPackage.feeds) {
				const promise = featuresApi.update_feed_contents(feedPackage.id, feed.id);
				promise.then(() => {
					haveTaskCompleted = true;
					list.notifyDatasourceUpdated(true);
				});
				tasks.addPending(`Schedule Updating For ${feedPackage.name} - ${feed.name}`, promise);
				try {
					await promise;
				} catch (e) {
					console.error(`Major schedule update failured for ${feedPackage.name} - ${feed.name}`);
				}
			}
		}
		if (haveTaskCompleted) {
			resolve!();
			list.notifyDatasourceUpdated(false);
		} else reject!();
	}

	function onSelectToday() {
		setCurrentFeedId(SPECIFY_FEED_IDSET.TODAY_FILTER);
	}

	function onSelectWeekend() {
		setCurrentFeedId(SPECIFY_FEED_IDSET.WEEKEND_FILTER);
	}

	function onSelectFavorite() {
		setCurrentFeedId(SPECIFY_FEED_IDSET.FAVORITE_FILTER);
	}

	function onSelectUnread() {
		setCurrentFeedId(SPECIFY_FEED_IDSET.UNREAD_FILTER);
	}

	async function setCurrentFeedId(value: string) {
		const feedPackage = feeds.findPackagesOwnerByFeedId(value);
		let packageId = undefined;
		const isFeedSpecified = isSpecifyFeed(value);
		if (!feedPackage) {
			if (!isFeedSpecified)
				return Promise.reject(
					`unexpect error, the package owner of feedId = ${value} was not found`
				);
			else packageId = 'VIRTUAL_NO_PACKAGE_SPECIFY_FEED';
		} else {
			packageId = feedPackage.id;
		}

		currentFeedId = value;
		const { list } = articles;
		list.associatedFeedId = currentFeedId;
		list.associatedPackageId = packageId;
		await list.refresh(true);
		currentArticle = list.groupedArticles[0]?.articles[0];

		if (!currentArticle) {
			if (!isFeedSpecified) {
				await list.updateFeeds();
			} else if (isRecentFamilyFeed(value)) {
				if (!globalSharedScheduleUpdatingFuture) scheduleUpdate();
				list.attachInitLoadingFuture(globalSharedScheduleUpdatingFuture!);
			}
		}
	}

	// 处理初始化流程
	$effect(() => {
		const { list } = articles;
		feeds.refresh().then(() => {
			setCurrentFeedId(SPECIFY_FEED_IDSET.TODAY_FILTER).then(() => {
				// 如果本次feed对应的内容不是当天且当天更新过则全力昂Feed更新。
				// TODO：应根据schedule最后更新日期判断。
				const lastest_article = list.groupedArticles[0]?.articles[0];
				if (
					!has_update_feeds_on_boot &&
					(!lastest_article || lastest_article?.created_at != currentDateText())
				) {
					has_update_feeds_on_boot = true;
					scheduleUpdate();
				}
			});
		});
	});

	return {
		get feeds() {
			return feeds;
		},
		get currentFeedId() {
			return currentFeedId;
		},
		get currentArticle() {
			return currentArticle;
		},
		get articles() {
			return articles;
		},
		get tasks() {
			return tasks;
		},
		get reader() {
			return reader;
		},
		get sprite() {
			return sprite;
		},
		setCurrentFeedId,
		setCurrentArticle,
		getCurrentArticle,
		scheduleUpdate,
		onSelectToday,
		onSelectWeekend,
		onSelectFavorite,
		onSelectUnread,
		get isTodaySelected() {
			return isTodaySelected;
		},
		get isWeekendSelected() {
			return isWeekendSelected;
		},
		get isFavoriteSelected() {
			return isFavoriteSelected;
		},
		get isUnreadSelected() {
			return isUnreadSelected;
		},
		get isFeedSpecified() {
			return isFeedSpecified;
		}
	};
}

export { createStore };
