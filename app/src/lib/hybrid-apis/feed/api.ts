import type { Option } from "../tauri-regular";
import type {
  AppConfig,
  ArticleModel,
  ConversationMessage,
  FeedsPackage,
  FeedTargetDescription,
} from "./types";

interface FeaturesAPI {
  add_feeds_package: (feeds_package: FeedsPackage) => Promise<void>;

  remove_feeds_package: (package_id: string) => Promise<void>;

  rename_feeds_package: (package_id: string, new_name: string) => Promise<void>;

  add_feed: (package_id: string, ftd: FeedTargetDescription) => Promise<void>;

  remove_feed: (package_id: string, feed_id: string) => Promise<void>;

  rename_feed: (
    package_id: string,
    feed_id: string,
    new_name: string,
  ) => Promise<void>;

  get_feeds_packages: () => Promise<FeedsPackage[]>;

  get_feeds_by_package: (package_id: string) => Promise<Option<FeedsPackage>>;

  update_feed_contents: (package_id: string, feed_id: string) => Promise<void>;

  read_feed_contents: (
    feed_id: string,
    offset: number,
    count: number,
  ) => Promise<ArticleModel[]>;

  query_by_id: (id: number) => Promise<ArticleModel>;

  get_app_config: () => Promise<AppConfig>;

  set_app_config: (appConfig: AppConfig) => Promise<void>;

  get_ollama_status: () => Promise<void>;

  download_ollama: () => Promise<void>;

  launch_ollama: () => Promise<void>;

  open_article_external: (url: string) => Promise<void>;

  scrap_text_by_url: (url: string) => Promise<string>;

  update_article_by_source: (
    article_id: number,
    url: string,
  ) => Promise<boolean>;

  chat_with_article_assistant: (
    article_id: number,
    user_prompt: string,
    history: ConversationMessage[],
  ) => Promise<string>;

  search_contents_by_keyword: (
    keyword: string,
    offset: number,
    count: number,
  ) => Promise<ArticleModel[]>;
}

export type { FeaturesAPI };
