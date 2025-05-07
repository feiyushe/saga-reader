import { createToaster } from '@skeletonlabs/skeleton-svelte';

export const globalToaster = createToaster({
	placement: 'bottom-end',
	offsets: '30px'
});
export const spriteToaster = createToaster({
	placement: 'bottom-start',
	offsets: '128px'
});
