use std::env;
use std::process::Command;

use fslock::LockFile;
use spdlog::{error, info};

use crate::daemon::locks::get_lock_path;

pub fn launch_ignore_error(host_arg: &str, locker_monitor_name: &str) -> () {
    match launch(host_arg, locker_monitor_name) {
        Ok(_) => (),
        Err(e) => error!(
            "daemon launch error, host_arg = {}, locker_monitor_path = {}, error = {}",
            host_arg, locker_monitor_name, e
        ),
    }
}

pub fn launch(host_arg: &str, locker_monitor_name: &str) -> anyhow::Result<()> {
    let lock_path = get_lock_path(locker_monitor_name);
    let mut updater_lock = LockFile::open(&lock_path)?;
    if !updater_lock.try_lock()? {
        info!(
            "it seems the daemon has already launched because of the locker has already owned. host_arg = {}, locker_monitor_path = {}",
            host_arg, locker_monitor_name
        );
        return Ok(());
    }
    updater_lock.unlock()?;
    info!(
        "daemon launching, host_arg = {}, locker_monitor_path = {}",
        host_arg, locker_monitor_name
    );
    let current_exe = env::current_exe()?;
    Command::new(current_exe).arg(host_arg).spawn()?;
    info!(
        "daemon launched, host_arg = {}, locker_monitor_path = {}",
        host_arg, locker_monitor_name
    );
    Ok(())
}
