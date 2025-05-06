import { create as createSearch } from './search/index.svelte';
import { create as createList } from './list/index.svelte';
import type { StoreType as TasksStoreType } from '../tasks.svelte';

export function create(tasks: TasksStoreType) {
	const search = createSearch();
	const list = createList({
		tasks,
		search
	});

	return {
		search,
		list
	};
}
