use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use recorder::path::get_appdata_file;
use types::{FeedsPackage, FeedTargetDescription, UserConfig};

use super::task::{InitTask, TaskInitializer};

const FILE_NAME_USER_CONFIG: &str = "user_config.toml";

pub async fn call() -> anyhow::Result<InitTask<UserConfig>> {
    let mut task = InitTask::default();
    task.start("user_profile", || async {
        let user_config_path = get_appdata_file(FILE_NAME_USER_CONFIG);
        Ok(match File::open(user_config_path).await {
            Ok(mut file) => {
                let mut data_raw = String::new();
                file.read_to_string(&mut data_raw).await?;
                toml::from_str(data_raw.as_str())?
            }
            Err(_) => {
                let user_profile = default_user_profile();
                sync_to(&user_profile).await?;
                user_profile
            }
        })
    })
        .await?;
    Ok(task)
}

pub async fn sync_to(user_config: &UserConfig) -> anyhow::Result<()> {
    let user_config_path = get_appdata_file(FILE_NAME_USER_CONFIG);
    let mut file = File::create(user_config_path).await?;
    let data_raw = toml::to_string(user_config)?;
    file.write_all(data_raw.as_bytes()).await?;
    Ok(())
}

fn default_user_profile() -> UserConfig {
    UserConfig {
        feeds_packages: vec![FeedsPackage {
            name: "未分类".into(),
            feeds: vec![FeedTargetDescription {
                id: "default_flat_on_root".into(),
                fetcher_id: "scrap".into(),
                name: "".into(),
                data: vec!["英伟达".into(), "投资".into()],
            }],
            is_flat_on_root: true,
            id: "".into(),
        }],
    }
}
