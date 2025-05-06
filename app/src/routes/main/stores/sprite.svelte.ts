import type {
  ConversationInput,
  ConversationMessage,
} from "$lib/hybrid-apis/feed/types";
import {
  Status,
  create as createLoadingStore,
  type StoreType as LoadingStore,
} from "./loading.svelte";
import { featuresApi } from "$lib/hybrid-apis/feed/impl";
import type { IContext } from "./context";

type StoreType = {
  opened: boolean;
  loading: LoadingStore;
  toggle: () => void;
  history: ConversationMessage[];
  send: (input: ConversationInput) => Promise<boolean>;
  isLoading: boolean;
};

function create(context: IContext): StoreType {
  let opened = $state(false);
  let history = $state<ConversationMessage[]>([]);
  const loading = createLoadingStore();

  function cleanUp() {
    history = [];
    loading.unset();
  }

  async function send(input: ConversationInput): Promise<boolean> {
    if (loading.status === Status.Loading) return false;

    loading.load();

    try {
      const message: ConversationMessage = {
        role: "user",
        mtype: input.mtype,
        payload: input.payload,
        created_at: `${Date.now()}`,
      };
      history.push(message);

      const replyText = await featuresApi.chat_with_article_assistant(
        context.currentArticle?.id,
        message.payload,
        history,
      );

      const replyMessage: ConversationMessage = {
        role: "system",
        mtype: "text",
        payload: replyText,
        created_at: `${Date.now()}`,
      };
      history.push(replyMessage);
      loading.complete();
      return true;
    } catch (e) {
      console.error("error occurs when the store.send executing", e);
      loading.error(new Error(String(e)));
      return false;
    }
  }

  return {
    get loading() {
      return loading;
    },
    get opened() {
      return opened;
    },
    get history() {
      return history;
    },
    toggle: () => {
      opened = !opened;
      if (!opened) cleanUp();
    },
    get isLoading() {
      return loading.status === Status.Loading;
    },
    send,
  };
}

export { create };
export type { StoreType };
