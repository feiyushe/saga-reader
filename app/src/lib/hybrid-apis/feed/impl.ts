import { call } from "../tauri-regular";
import type { Option } from "../tauri-regular";
import type { FeaturesAPI } from "./api";
import type {
  FeedsPackage,
  FeedTargetDescription,
  ArticleModel,
  AppConfig,
  ConversationMessage,
} from "./types";

class FeaturesAPIImpl implements FeaturesAPI {
  add_feeds_package(feeds_package: FeedsPackage): Promise<void> {
    return call("add_feeds_package", { feeds_package });
  }

  remove_feeds_package(package_id: string): Promise<void> {
    return call("remove_feeds_package", { package_id });
  }

  rename_feeds_package(package_id: string, new_name: string): Promise<void> {
    return call("rename_feeds_package", { package_id, new_name });
  }

  add_feed(package_id: string, ftd: FeedTargetDescription): Promise<void> {
    return call("add_feed", { package_id, ftd });
  }

  remove_feed(package_id: string, feed_id: string): Promise<void> {
    return call("remove_feed", { package_id, feed_id });
  }

  rename_feed(
    package_id: string,
    feed_id: string,
    new_name: string,
  ): Promise<void> {
    return call("rename_feed", { package_id, feed_id, new_name });
  }

  change_feed_data(
    package_id: string,
    feed_id: string,
    data: string[],
  ): Promise<void> {
    return call("change_feed_data", { package_id, feed_id, data });
  }

  get_feeds_packages(): Promise<FeedsPackage[]> {
    return call("get_feeds_packages", {});
  }

  get_feeds_by_package(package_id: string): Promise<Option<FeedsPackage>> {
    return call("get_feeds_by_package", { package_id });
  }

  update_feed_contents(package_id: string, feed_id: string): Promise<void> {
    return call("update_feed_contents", { package_id, feed_id });
  }

  read_feed_contents(
    feed_id: string,
    offset: number,
    count: number,
  ): Promise<ArticleModel[]> {
    return call("read_feed_contents", { feed_id, offset, count });
  }

  query_by_id(id: number): Promise<ArticleModel> {
    return call("query_by_id", { id });
  }

  mark_as_read(id: number): Promise<void> {
    return call("mark_as_read", { id });
  }

  get_app_config(): Promise<AppConfig> {
    return call("get_app_config", {});
  }

  set_app_config(app_config: AppConfig): Promise<void> {
    return call("set_app_config", { app_config });
  }

  get_ollama_status(): Promise<void> {
    return call("get_ollama_status", {});
  }

  download_ollama(): Promise<void> {
    return call("download_ollama", {});
  }

  launch_ollama(): Promise<void> {
    return call("launch_ollama", {});
  }

  open_article_external(url: string): Promise<void> {
    return call("open_article_external", { url });
  }

  set_favorite(id: number, favorite: boolean): Promise<boolean> {
    return call("set_favorite", { id, favorite });
  }

  scrap_text_by_url(url: string): Promise<string> {
    return call("scrap_text_by_url", { url });
  }

  update_article_by_source(article_id: number, url: string): Promise<boolean> {
    return call("update_article_by_source", { article_id, url });
  }

  chat_with_article_assistant(
    article_id: number | undefined,
    user_prompt: string,
    history: ConversationMessage[],
  ): Promise<string> {
    return call("chat_with_article_assistant", {
      article_id,
      user_prompt,
      history,
    });
  }

  search_contents_by_keyword(
    keyword: string,
    offset: number,
    count: number,
  ): Promise<ArticleModel[]> {
    return call("search_contents_by_keyword", { keyword, offset, count });
  }
}

const SPECIFY_FEED_IDSET = {
  TODAY_FILTER: "TODAY_FILTER",
  WEEKEND_FILTER: "WEEKEND_FILTER",
  FAVORITE_FILTER: "FAVORITE_FILTER",
  UNREAD_FILTER: "UNREAD_FILTER",
};

function isSpecifyFeed(feedId: string | undefined) {
  if (!feedId) return false;
  return (
    Object.values(SPECIFY_FEED_IDSET).findIndex((id) => id === feedId) >= 0
  );
}

function isRecentFamilyFeed(feedId: string | undefined) {
  if (!feedId) return false;
  return (
    feedId === SPECIFY_FEED_IDSET.TODAY_FILTER ||
    feedId === SPECIFY_FEED_IDSET.WEEKEND_FILTER
  );
}

const featuresApi = new FeaturesAPIImpl();

export {
  FeaturesAPIImpl,
  featuresApi,
  SPECIFY_FEED_IDSET,
  isSpecifyFeed,
  isRecentFamilyFeed,
};
