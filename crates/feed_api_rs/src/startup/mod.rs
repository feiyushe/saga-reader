use spdlog::{error, info};

use crate::application_context::{ApplicationContext, ContextHost};

pub mod init_app_config;
pub mod init_logger;
pub mod init_llm;
pub mod init_user_profile;
pub mod task;
mod types;

pub struct Startup {
    context: ApplicationContext,
}

impl Startup {
    pub async fn launch() -> anyhow::Result<Startup> {
        let context = tiger0_1().await?;
        Ok(Startup::new(context))
    }
}

impl ContextHost for Startup {
    fn new(context: ApplicationContext) -> Self {
        Startup { context }
    }

    fn get_context(&self) -> &ApplicationContext {
        &self.context
    }

    fn copy_context(&self) -> ApplicationContext {
        self.context.clone()
    }
}

pub async fn tiger0_1() -> anyhow::Result<ApplicationContext> {
    // 初始化Tiger0，即应用启动环节所必须且同步初始化的模块，如读取应用配置。
    // 获得应用配置
    let app_config = init_app_config::call().await?.result.unwrap();

    // 应用运行的初始化必要组件，如日志、监控等。
    init_logger::call(&app_config)?;
    info!("starting up...tiger0_1, application configuration and logger initialized");

    // 初始化Tiger1，即应用启动环节必须但可并行初始化的模块，如用户配置、依赖服务。
    info!("starting up...tiger0_1, begin initialize user configuration and ollama program status");
    let llm_section = &app_config.llm;
    let (r1, r2) = tokio::join!(
        // 初始化用户订阅数据
        init_user_profile::call(),
        // 初始化依赖服务
        init_llm::call(llm_section)
    );
    if r1.is_err() || r2.is_err() {
        error!("starting up...tiger0_1, error occurs...{:?}, {:?}", r1.as_ref().err(), r2.as_ref().err());
        return Err(anyhow::Error::msg(
            format!("error in startup..., {:?}, {:?}", r1.as_ref().err(), r2.as_ref().err())
        ));
    }

    info!("starting up...tiger0_1, end initialize");
    Ok(ApplicationContext {
        app_config,
        user_config: r1?.result.unwrap(),
    })
}

// 初始化Tiger2为应用启动环节可延迟初始化的模块，不再这里运行。
pub async fn tiger2(_context: ApplicationContext) -> anyhow::Result<ApplicationContext> {
    todo!();
}
