interface MarkdownProps {
	value: string;
}

interface EmbedWebViewProps {
	src: string;
	onLoadStart?: (src: string) => void;
	onLoadEnd?: (src: string) => void;
}

export type { EmbedWebViewProps, MarkdownProps };
