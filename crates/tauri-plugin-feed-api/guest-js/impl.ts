import { call } from './tauri-regular';
import type { Option } from './tauri-regular';
import type { FeaturesAPI } from './api';
import type { FeedsPackage, FeedTargetDescription, ArticleModel } from './types';

class FeaturesAPIImpl implements FeaturesAPI {
	add_feeds_package(feeds_package: FeedsPackage): Promise<void> {
		return call('add_feeds_package', { feeds_package });
	}

	remove_feeds_package(package_id: string): Promise<void> {
		return call('remove_feeds_package', { package_id });
	}

	rename_feeds_package(package_id: string, new_name: string): Promise<void> {
		return call('rename_feeds_package', { package_id, new_name });
	}

	add_feed(package_id: string, ftd: FeedTargetDescription): Promise<void> {
		return call('add_feed', { package_id, ftd });
	}

	remove_feed(package_id: string, feed_id: string): Promise<void> {
		return call('remove_feed', { package_id, feed_id });
	}

	rename_feed(package_id: string, feed_id: string, new_name: string): Promise<void> {
		return call('rename_feed', { package_id, feed_id, new_name });
	}

	get_feeds_packages(): Promise<FeedsPackage[]> {
		return call('get_feeds_packages', {});
	}

	get_feeds_by_package(package_id: string): Promise<Option<FeedsPackage>> {
		return call('get_feeds_by_package', { package_id });
	}

	update_feed_contents(package_id: string, feed_id: string): Promise<void> {
		return call('update_feed_contents', { package_id, feed_id });
	}

	read_feed_contents(feed_id: string, offset: number, count: number): Promise<ArticleModel[]> {
		return call('read_feed_contents', { feed_id, offset, count });
	}

	get_ollama_status(): Promise<void> {
		return call('get_ollama_status', {});
	}

	download_ollama(): Promise<void> {
		return call('download_ollama', {});
	}

	launch_ollama(): Promise<void> {
		return call('launch_ollama', {});
	}
}

const featuresApi = new FeaturesAPIImpl();

export { FeaturesAPIImpl, featuresApi };
