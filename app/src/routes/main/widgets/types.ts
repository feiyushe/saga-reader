import type { Article } from '$lib/types/article';
import type { StoreType as SearchStoreType } from '../stores/articles/search/index.svelte';
import type { StoreType as ListStoreType } from '../stores/articles/list/index.svelte';
import type { StoreType as TasksStoreType } from '../stores/tasks.svelte';
import type { StoreType as FeedsStoreType } from '../stores/feeds.svelte';
import type { StoreType as ReaderStoreType } from '../stores/reader.svelte';
import type { StoreType as AISpriteStore, DefaultQuestion } from '../stores/sprite.svelte';

interface FeedsListProps {
    store: FeedsStoreType;
    selectedFeedId: string | undefined;
    onFeedPressed: (feedId: string) => void;
    onSelectToday: () => void;
    onSelectWeekend: () => void;
    isTodaySelected: boolean;
    isWeekendSelected: boolean;
    onSelectFavorite: () => void;
    isFavoriteSelected: boolean;
    onSelectUnread: () => void;
    isUnreadSelected: boolean;
    onExpandFeedGroup?: (feedId: string) => void;
}

interface SearchBarProps {
	store: SearchStoreType;
	articles_store: ListStoreType;
}

interface ArticleReaderProps {
    articleId: number;
    store: ReaderStoreType;
    onFeedTagClick?: (feedName: string) => void;
}

interface ArticlesGroup {
	name: string;
	articles: Article[];
}

interface ArticlesListProps {
	store: ListStoreType;
	markAsRead: (articleId: number) => Promise<void>;
	isFilterActived: boolean;
	isFeedSpecified: boolean;
	selectedArticle: Article | null;
	onArticlePressed: (article: Article) => void;
}

interface FooterProps {
	tasksStore: TasksStoreType;
}

interface AISpriteProps {
	store: AISpriteStore;
}

type ArticleReadMode = 'optimized' | 'melted' | 'original';

export type {
	FeedsListProps,
	ArticleReaderProps,
	ArticleReadMode,
	ArticlesGroup,
	ArticlesListProps,
	SearchBarProps,
	FooterProps,
	AISpriteProps
};
