import { featuresApi } from "$lib/hybrid-apis/feed/impl";
import type { Article } from "$lib/types/article";
import { Status } from "./loading.svelte";
import type { StoreType as TasksStoreType } from "./tasks.svelte";

type Associates = {
  tasks: TasksStoreType;
};

type StoreType = {
  markAsRead: (articleId: number) => Promise<void>;
  refreshByEnhancedScraper: (
    articleId: number,
    url: string,
  ) => Promise<Article>;
};

function create(associates: Associates): StoreType {
  const { tasks } = associates;
  function markAsRead(articleId: number) {
    return featuresApi.mark_as_read(articleId);
  }

  async function refreshByEnhancedScraper(
    articleId: number,
    url: string,
  ): Promise<Article> {
    const taskId = `Article Updating For ArticleID = ${articleId}`;
    const pending = tasks.queryPending(taskId);
    if (pending) {
      if (pending.loadingStore.status === Status.Error) {
        tasks.remove(pending);
      } else {
        return pending.promise as Promise<Article>;
      }
    }

    const promise = featuresApi
      .update_article_by_source(articleId, url)
      .then(() => featuresApi.query_by_id(articleId));

    tasks.addPending(taskId, promise);
    return promise;
  }

  return {
    markAsRead,
    refreshByEnhancedScraper,
  };
}

export type { StoreType };
export { create };
