use types::{AppConfig, UserConfig};

// const version: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct ApplicationContext {
    pub app_config: AppConfig,
    pub user_config: UserConfig,
}

pub struct ContextHostWrapper {
    context: ApplicationContext,
}

pub trait ContextHost {
    fn new(context: ApplicationContext) -> Self;

    fn get_context(&self) -> &ApplicationContext;

    fn copy_context(&self) -> ApplicationContext;
}

impl ContextHost for ContextHostWrapper {
    fn new(context: ApplicationContext) -> Self {
        ContextHostWrapper { context }
    }

    fn get_context(&self) -> &ApplicationContext {
        &self.context
    }

    fn copy_context(&self) -> ApplicationContext {
        self.context.clone()
    }
}
