use crate::daemon::args::DAEMON_FEEDS_SCHEDULE_UPDATE;

pub fn is_daemon() -> bool {
    let launch_mode = std::env::args().nth(1).unwrap_or_default();
    launch_mode.eq(DAEMON_FEEDS_SCHEDULE_UPDATE)
}
