import type { ArticleModel, FeedsPackage, FeedTargetDescription } from './types';

interface FeaturesAPI {
	add_feeds_package: (feeds_package: FeedsPackage) => Promise<void>;

	remove_feeds_package: (package_id: string) => Promise<void>;

	rename_feeds_package: (package_id: string, new_name: string) => Promise<void>;

	add_feed: (package_id: string, ftd: FeedTargetDescription) => Promise<void>;

	remove_feed: (package_id: string, feed_id: string) => Promise<void>;

	rename_feed: (package_id: string, feed_id: string, new_name: string) => Promise<void>;

	get_feeds_packages: () => Promise<FeedsPackage[]>;

	get_feeds_by_package: (package_id: string) => Promise<Option<FeedsPackage>>;

	update_feed_contents: (package_id: string, feed_id: string) => Promise<void>;

	read_feed_contents: (feed_id: string, offset: number, count: number) => Promise<ArticleModel[]>;

	query_by_id(id: i32) -> anyhow::Result<Option<ArticleModel>>;

	get_ollama_status: () => Promise<void>;

	download_ollama: () => Promise<void>;

	launch_ollama: () => Promise<void>;
}

export type { FeaturesAPI };
