use std::sync::Arc;
use std::time::Duration;

use spdlog::{
    Logger,
    prelude::*,
    sink::{RotatingFileSink, RotationPolicy},
};
use spdlog::formatter::{pattern, PatternFormatter};

use recorder::path::get_appdata_file_in_dir;
use types::{AppConfig, AppConfigLogSection, OutputType};

pub fn call(app_config: &AppConfig) -> anyhow::Result<()> {
    let log_section = &app_config.log;
    init_by(log_section)
}

pub fn init_by(log_section: &AppConfigLogSection) -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        specify_logger_format(&spdlog::default_logger());
        spdlog::default_logger().set_level_filter(LevelFilter::All);
        info!("auto turn log on because of debug_assertions is active");
        return Ok(());
    }
    if !log_section.enable {
        spdlog::default_logger().set_level_filter(LevelFilter::Off);
        return Ok(());
    }
    match log_section.output_type {
        OutputType::Stdout => {
            specify_logger_format(&spdlog::default_logger());
            info!("the log was enabled and would be output to stdout");
            Ok(())
        }
        OutputType::Disk => {
            let path = get_appdata_file_in_dir("logs", format!("app{}.log", log_section.log_name_tail));
            info!("the log was enabled in disk mode and would be recorded in {:?} with date-named mode", path);
            let new_logger = spdlog::default_logger().fork_with(|new| {
                let file_sink = Arc::new(
                    RotatingFileSink::builder()
                        .base_path(path)
                        .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
                        .build()?,
                );
                new.sinks_mut().push(file_sink);
                Ok(())
            })?;
            new_logger.set_flush_period(Some(Duration::from_secs(3)));
            spdlog::set_default_logger(new_logger);
            specify_logger_format(&spdlog::default_logger());
            info!("the log was enabled and would be recorded in the log file");
            Ok(())
        }
        OutputType::UnSpecified => {
            specify_logger_format(&spdlog::default_logger());
            info!("the log was enabled and would be output to stdout");
            Ok(())
        }
    }
}

/// 指定logger的format。
///
/// formatter格式定义见[源码](https://github.com/SpriteOvO/spdlog-rs/blob/aa10020e305352a77f302e6737ecf114548013bb/spdlog-internal/src/pattern_parser/mod.rs#L88)
fn specify_logger_format(logger: &Arc<Logger>) {
    let new_formatter = Box::new(PatternFormatter::new(pattern!(
        "[{date} {time}.{millisecond}] [thread-{tid}] [{^{level}}] {payload}{eol}"
    )));

    for sink in logger.sinks() {
        sink.set_formatter(new_formatter.clone())
    }
}