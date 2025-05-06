import { createToaster } from '@skeletonlabs/skeleton-svelte';

export const globalToaster = createToaster({
	offsets: '36px'
});
export const spriteToaster = createToaster({
	placement: 'bottom-start',
	offsets: '128px'
});
