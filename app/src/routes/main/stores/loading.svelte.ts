type StoreType = {
	unset: () => void;
	load: () => void;
	complete: () => void;
	error: (e: Error) => void;
	status: Status;
	statusText: string;
};

enum Status {
	Unset,
	Loading,
	Completed,
	Error
}

function create(initStatus = Status.Unset): StoreType {
	let status: Status = $state(initStatus);

	function unset() {
		status = Status.Unset;
	}

	function load() {
		status = Status.Loading;
	}

	function error(e: Error) {
		console.error('LoadingStore state error', e);
		status = Status.Error;
	}

	function complete() {
		status = Status.Completed;
	}

	return {
		unset,
		load,
		error,
		complete,
		get status() {
			return status;
		},
		get statusText() {
			switch (status) {
				case Status.Unset:
					return '未设定状态';
				case Status.Loading:
					return '执行中';
				case Status.Completed:
					return '完成';
				case Status.Error:
					return '错误';
			}
		}
	};
}

export type { StoreType };
export { Status, create };
