interface Article {
    id: number;
    title: string;
    head_read: string;
    published_at: string;
    has_read: boolean;
    source_link: string;
    purged_content: string;
    optimized_content: string;
    melted_content: string;
    created_at: string;
    group_id: string;
    is_favorite: boolean;
    feed_name?: string;
}

export type { Article };
