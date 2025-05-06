import { browser } from '$app/environment';

function disableContextMenu(node: Document | HTMLElement) {
	if (!browser) return;
	const handler = (e: Event) => {
		const target = e.target;
		if (target instanceof HTMLInputElement) {
			return;
		}
		e.preventDefault();
	};
	node.addEventListener('contextmenu', handler, false);
	return {
		destroy() {
			node.removeEventListener('contextmenu', handler, false);
		}
	};
}

type ObserveVisiblityOption = {
	callback: (v: boolean) => void;
};

function observeVisiblity(node: HTMLElement, opt: ObserveVisiblityOption) {
	const { callback } = opt;
	const observer = new IntersectionObserver((entries) => {
		entries.forEach((entry) => {
			if (entry.isIntersecting) {
				callback(true);
			} else {
				callback(false);
			}
			return;
		});
	});

	observer.observe(node);

	return {
		destroy() {
			observer.unobserve(node);
		}
	};
}

export { disableContextMenu, observeVisiblity };
