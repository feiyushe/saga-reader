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

type DefaultQuestion = {
  id: string;
  text: string;
  category?: string;
};

type StoreType = {
  opened: boolean;
  loading: LoadingStore;
  toggle: () => void;
  history: ConversationMessage[];
  send: (input: ConversationInput) => Promise<boolean>;
  sendDefaultQuestion: (question: DefaultQuestion) => Promise<boolean>;
  isLoading: boolean;
  hasConversation: boolean;
  getDefaultQuestions: () => DefaultQuestion[];
};

function create(context: IContext): StoreType {
  let opened = $state(false);
  let history = $state<ConversationMessage[]>([]);
  const loading = createLoadingStore();

  function cleanUp() {
    history = [];
    loading.unset();
  }

  // 默认问题推荐
  function getDefaultQuestions(): DefaultQuestion[] {
    return [
      {
        id: "summarize",
        text: "请总结这篇文章的核心观点",
        category: "analysis"
      },
      {
        id: "key_points",
        text: "这篇文章的关键信息有哪些？",
        category: "analysis"
      },
      {
        id: "explain_concept",
        text: "请解释文章中的重要概念",
        category: "explanation"
      },
      {
        id: "practical_application",
        text: "这些内容在实际中如何应用？",
        category: "application"
      },
      {
        id: "related_topics",
        text: "有哪些相关的话题值得深入了解？",
        category: "exploration"
      },
      {
        id: "critical_thinking",
        text: "对这篇文章的观点有什么不同看法？",
        category: "critical"
      }
    ];
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

  async function sendDefaultQuestion(question: DefaultQuestion): Promise<boolean> {
    return await send({
      mtype: "text",
      payload: question.text
    });
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
    get hasConversation() {
      return history.length > 0;
    },
    toggle: () => {
      opened = !opened;
      if (!opened) cleanUp();
    },
    get isLoading() {
      return loading.status === Status.Loading;
    },
    send,
    sendDefaultQuestion,
    getDefaultQuestions,
  };
}

export { create };
export type { StoreType, DefaultQuestion };
