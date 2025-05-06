type StoreType = {
	filterText: string;
	isFilterActived: boolean;
};

function create(): StoreType {
	let filterText = $state('');
	const isFilterActived = $derived(filterText != '');

	return {
		get filterText() {
			return filterText;
		},
		set filterText(value) {
			filterText = value;
		},
		get isFilterActived() {
			return isFilterActived;
		}
	};
}

export type { StoreType };
export { create };
