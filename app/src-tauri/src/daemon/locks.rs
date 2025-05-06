use std::path::PathBuf;

use recorder::path::get_appdata_file_in_dir;

pub const LOCK_FEEDS_SCHEDULE_UPDATE: &str = "feeds_schedule_update.lock";

pub fn get_lock_path(locker_name: &str) -> PathBuf {
    get_appdata_file_in_dir("daemons", locker_name)
}
