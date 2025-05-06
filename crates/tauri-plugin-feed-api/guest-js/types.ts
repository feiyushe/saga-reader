type FeedTargetDescription = {
	id: string;
	name: string;
	fetcher_id: string;
	data: string[];
};

type FeedsPackage = {
	id: string;
	name: string;
	feeds: FeedTargetDescription[];
	is_flat_on_root: boolean;
};

type ArticleModel = {
	id: number;
	source_link: string;
	title: string;
	purged_content: string;
	optimized_content: string;
	melted_content: string;
	published_at: number;
	created_at: number;
	has_read: boolean;
	group_id: string;
};

export type { FeedsPackage, FeedTargetDescription, ArticleModel };
