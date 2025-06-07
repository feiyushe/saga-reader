import { featuresApi, isSpecifyFeed } from '$lib/hybrid-apis/feed/impl';
import type { Article } from '$lib/types/article';
import { tick } from 'svelte';
import type { ArticlesGroup } from '../../../widgets/types';
import { create as createLoading, Status } from '../../loading.svelte';
import type { StoreType as LoadingStore } from '../../loading.svelte';
import type { StoreType as SearchStore } from '../search/index.svelte';
import type { StoreType as TasksStoreType } from '../../tasks.svelte';

const PAGING_SIZE = 20;
const FEED_ID_DEFAULT_FLAT_ON_ROOT = 'default_flat_on_root';

type StoreType = {
	groupedArticles: ArticlesGroup[];
	filteredArticles: ArticlesGroup[];
	associatedPackageId: string | undefined;
	associatedFeedId: string | undefined;
	articles_init_loading: LoadingStore;
	articles_continous_loading: LoadingStore;
	filtered_articles_loading: LoadingStore;
	refresh: (waitUpdatePending: boolean) => Promise<unknown>;
	updateFeeds: () => Promise<unknown>;
	loadMore: () => void;
	notifyDatasourceUpdated: (continueLoading: boolean) => void;
	attachInitLoadingFuture: (future: Promise<void>) => void;
	isFeedSpecified: boolean;
};

type Associations = {
	tasks: TasksStoreType;
	search: SearchStore;
};

function create(associations: Associations): StoreType {
	let associatedPackageId: string | undefined = $state();
	let associatedFeedId = $state(FEED_ID_DEFAULT_FLAT_ON_ROOT);
	let groupedArticles: ArticlesGroup[] = $state([]);
	let filteredArticles: ArticlesGroup[] = $state([]);
	const articles_init_loading = createLoading();
	const articles_continous_loading = createLoading(Status.Completed);
	const filtered_articles_loading = createLoading();
	const searchAssociator: SearchStore = $state(associations.search);
	const isFeedSpecified = $derived(isSpecifyFeed(associatedFeedId));

	$effect(() => {
		let filterText = searchAssociator.filterText;
		tick().then(() => {
			filterText = searchAssociator.filterText;
			if (filterText === '') {
				filteredArticles = [];
				return;
			}
			const resultGroup = {
				name: '搜索结果',
				articles: [] as Article[]
			};

			featuresApi
				.search_contents_by_keyword(filterText, 0, 10000)
				.then((articles) => {
					resultGroup.articles.push(...articles);
					filteredArticles = [resultGroup];
				})
				.catch((e) => {
					console.error(e);
					filteredArticles = [resultGroup];
				});
		});
	});

	function notifyDatasourceUpdated(continueLoading: boolean = false) {
		// TODO:当底层数据更新时，期望的是尝试拉取比当前列表更新的数据项，而非刷新并重制当前列表。
		refresh(false, continueLoading);
	}

	async function updateFeeds(): Promise<unknown> {
		if (!associatedPackageId || !associatedFeedId)
			return Promise.reject(
				`then package id or feed id is null when updateFeeds called, packageId = ${associatedPackageId}, feedId = ${associatedFeedId}`
			);

		articles_init_loading.load();
		const taskId = generateTaskIdForUpdateFeed(associatedFeedId);
		const { tasks } = associations;
		const pending = tasks.queryPending(taskId);
		if (pending) {
			pending.promise.then(() => refresh(false)).catch((e) => articles_init_loading.error(e));
			return pending.promise;
		}
		const promise = featuresApi
			.update_feed_contents(associatedPackageId, associatedFeedId)
			.then(() => refresh(false))
			.catch((e) => articles_init_loading.error(e));
		tasks.addPending(taskId, promise);
		return promise;
	}

	function attachInitLoadingFuture(future: Promise<void>) {
		console.log('attachInitLoadingFuture...begin');
		articles_init_loading.load();
		future
			.then(() => console.log('attachInitLoadingFuture...end by then'))
			.then(() => refresh(true))
			.catch((e) => {
				articles_init_loading.error(e);
				console.log('attachInitLoadingFuture...end by error', e);
			});
	}

	async function refresh(waitUpdatePending = false, continueLoading = false) {
		return featuresApi
			.read_feed_contents(associatedFeedId, 0, PAGING_SIZE)
			.then((data) => {
				const pending_groupedArticles = [];
				let rolling_published_at = null;
				let rolling_group = null;
				for (const article of data) {
					const published_at = article.published_at;
					if (published_at !== rolling_published_at) {
						rolling_published_at = published_at;
						rolling_group = {
							name: published_at,
							articles: [article]
						};
						pending_groupedArticles.push(rolling_group);
						continue;
					}
					rolling_group?.articles.push(article);
				}
				groupedArticles = pending_groupedArticles;

				const { tasks } = associations;
				const taskId = generateTaskIdForUpdateFeed(associatedFeedId);
				const pending = tasks.queryPending(taskId);
				if (waitUpdatePending && pending) {
					articles_init_loading.load();
					pending.promise.then(articles_init_loading.complete).catch(articles_init_loading.error);
					return pending.promise;
				}
				if (!continueLoading) {
					articles_init_loading.complete();
				}
			})
			.catch((e) => articles_init_loading.error(e));
	}

	function loadMore() {
		if (articles_continous_loading.status === Status.Loading) return;

		articles_continous_loading.load();
		let rolling_group = groupedArticles[groupedArticles.length - 1];
		let rolling_published_at = rolling_group.name;
		const offset = groupedArticles
			.map((ag) => ag.articles.length)
			.reduce((accumulator, currentValue) => accumulator + currentValue);
		featuresApi.read_feed_contents(associatedFeedId, offset, PAGING_SIZE).then((data) => {
			articles_continous_loading.complete();
			for (const article of data) {
				const published_at = article.published_at;
				if (published_at !== rolling_published_at) {
					rolling_published_at = published_at;
					rolling_group = {
						name: published_at,
						articles: [article]
					};
					groupedArticles.push(rolling_group);
					continue;
				}
				rolling_group?.articles.push(article);
			}
			groupedArticles = groupedArticles.map((ag) => {
				return { ...ag };
			});
		});
	}

	return {
		get associatedPackageId() {
			return associatedPackageId!;
		},
		set associatedPackageId(val: string) {
			associatedPackageId = val;
		},
		get associatedFeedId() {
			return associatedFeedId;
		},
		set associatedFeedId(value: string) {
			associatedFeedId = value;
		},
		get groupedArticles() {
			return groupedArticles;
		},
		get filteredArticles() {
			return filteredArticles;
		},
		get articles_init_loading() {
			return articles_init_loading;
		},
		get articles_continous_loading() {
			return articles_continous_loading;
		},
		get filtered_articles_loading() {
			return filtered_articles_loading;
		},
		refresh,
		updateFeeds,
		loadMore,
		notifyDatasourceUpdated,
		attachInitLoadingFuture,
		get isFeedSpecified() {
			return isFeedSpecified;
		}
	};
}

function generateTaskIdForUpdateFeed(associatedFeedId: string) {
	return `Feed Updating For Feed ID = ${associatedFeedId}`;
}

export type { StoreType };
export { create };
