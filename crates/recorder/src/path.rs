use std::fs;
use std::path::{Path, PathBuf};

const FOLDER_NAME_APP_DATA: &str = "qino_feed.app_data";
const FILE_NAME_DB_RECORD: &str = "article_recorder.db";

/// 获取Recorder数据库文件路径
pub fn get_appdata_articles() -> PathBuf {
    ensure_app_data_prepared().join(FILE_NAME_DB_RECORD)
}

/// 获取App Data下指定文件的路径
pub fn get_appdata_file<P: AsRef<Path>>(file_name: P) -> PathBuf {
    ensure_app_data_prepared().join(file_name)
}

/// 获取App Data下指定文件夹下的指定文件的路径
pub fn get_appdata_file_in_dir<P: AsRef<Path>>(sub_dir_name: &str, file_name: P) -> PathBuf {
    ensure_dir_in_appdata_prepared(sub_dir_name).join(file_name)
}

/// 确保appdata下的应用文件夹内的子文件夹存在，如果不存在会自动创建指定路径的文件夹
fn ensure_dir_in_appdata_prepared(sub_dir_name: &str) -> PathBuf {
    let app_data_dir = dirs::data_local_dir()
        .unwrap()
        .join(FOLDER_NAME_APP_DATA)
        .join(sub_dir_name);
    ensure_dir_prepared(app_data_dir)
}

/// 确保appdata下的应用文件夹存在，如果不存在会自动创建指定路径的文件夹
fn ensure_app_data_prepared() -> PathBuf {
    let app_data_dir = dirs::data_local_dir().unwrap().join(FOLDER_NAME_APP_DATA);
    ensure_dir_prepared(app_data_dir)
}

/// 确保给定的路径文件夹存在，如果不存在会自动创建指定路径的文件夹
fn ensure_dir_prepared(dir_path: PathBuf) -> PathBuf {
    let dir_path_meta = fs::metadata(&dir_path);
    if dir_path_meta.is_ok() && dir_path_meta.unwrap().is_dir() {
        return dir_path;
    }
    fs::create_dir(&dir_path).unwrap();
    dir_path
}
