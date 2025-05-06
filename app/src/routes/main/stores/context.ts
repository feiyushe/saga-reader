import type { Article } from '$lib/types/article';

interface IContext {
	currentFeedId: string | undefined;
	currentArticle: Article | null;
}

export type { IContext };
