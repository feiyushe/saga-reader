interface MarkdownProps {
	value: string;
}

type ArticleRenderProps = MarkdownProps;
type ArticleRenderType = 'markdown' | 'html';

interface EmbedWebViewProps {
	src: string;
	onLoadStart?: (src: string) => void;
	onLoadEnd?: (src: string) => void;
}

export type { EmbedWebViewProps, MarkdownProps, ArticleRenderProps, ArticleRenderType };
