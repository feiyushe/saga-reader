use feed_api_rs::features::impl_default::FeaturesAPIImpl;

pub(crate) struct HybridRuntimeState {
    pub features_api: FeaturesAPIImpl,
}