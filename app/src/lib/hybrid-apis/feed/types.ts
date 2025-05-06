type LLMProviderType = 'ollama' | 'platform' | 'glm';

type LLMSection = {
	provider_ollama: {
		endpoint: {
			api_base_url: string;
			model: string;
		};
	};
	provider_glm: {
		api_key: string;
	};
	provider_platform: {
		template_path: string;
		model_path: string;
	};
	active_provider_type: LLMProviderType;
};

type AppConfig = {
	llm: LLMSection;
	scrap: 'baidu' | 'bing';
	daemon: {
		frequency_feeds_update: boolean;
	};
};

type FeedFetcherID = 'scrap' | 'rss';

type FeedTargetDescription = {
	id: string;
	name: string;
	fetcher_id: FeedFetcherID;
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
	title: string;
	source_link: string;
	head_read: string;
	purged_content: string;
	optimized_content: string;
	melted_content: string;
	published_at: string;
	created_at: string;
	has_read: boolean;
	group_id: string;
	is_favorite: boolean;
};

type ConversationMessage = {
	role: 'system' | 'user' | 'assistant';
	mtype: 'text' | 'image' | 'video' | 'audio' | 'file';
	payload: string;
	created_at: string;
};

type ConversationInput = {
	mtype: 'text' | 'image' | 'video' | 'audio' | 'file';
	payload: string;
};

export type {
	AppConfig,
	LLMProviderType,
	FeedsPackage,
	FeedTargetDescription,
	ArticleModel,
	FeedFetcherID,
	ConversationMessage,
	ConversationInput
};
