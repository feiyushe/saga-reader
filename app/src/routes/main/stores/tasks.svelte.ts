import type { StoreType as LoadingStore } from './loading.svelte';
import { create as createLoadingStore, Status } from './loading.svelte';

type PendingItem = {
	description: string;
	loadingStore: LoadingStore;
	promise: Promise<unknown>;
};

type StoreType = {
	pendingStatus: Status;
	pendingStatusText: string;
	pendings: PendingItem[];
	addPending: (description: string, promise: Promise<unknown>) => void;
	queryPending: (description: string) => PendingItem | undefined;
	remove: (pending: PendingItem) => void;
};

function createPending(description: string, promise: Promise<unknown>): PendingItem {
	const loadingStore = createLoadingStore(Status.Loading);
	promise.then(loadingStore.complete).catch(loadingStore.error);
	return {
		description,
		loadingStore,
		promise
	};
}

function create(): StoreType {
	let pendings: PendingItem[] = $state([]);
	const pendingStatus: Status = $derived.by(() => {
		let hasError = false;
		for (const pending of pendings) {
			if (pending.loadingStore.status === Status.Loading) return Status.Loading;
			if (pending.loadingStore.status === Status.Error) hasError = true;
		}
		return hasError ? Status.Error : Status.Completed;
	});

	const pendingStatusText = $derived.by(() => {
		switch (pendingStatus) {
			case Status.Loading:
				const loadingPendings = pendings.filter((p) => p.loadingStore.status === Status.Loading);
				return `处理中...${loadingPendings.length}项`;
			case Status.Completed:
				return '就绪';
			case Status.Error:
				return '出现错误，点击查看详情';
		}
	});

	function remove(p: PendingItem) {
		pendings = pendings.filter((pending) => pending !== p);
	}

	function addPending(description: string, promise: Promise<unknown>) {
		const pending = createPending(description, promise);
		promise.then(() => {
			pendings = pendings.filter((pending) => pending.loadingStore.status !== Status.Completed);
		});
		pendings.push(pending);
	}

	function queryPending(description: string) {
		return pendings.find((pending) => pending.description === description);
	}

	return {
		get pendingStatus() {
			return pendingStatus;
		},
		get pendingStatusText() {
			return pendingStatusText;
		},
		get pendings() {
			return pendings;
		},
		addPending,
		queryPending,
		remove
	};
}

export type { StoreType };
export { create };
