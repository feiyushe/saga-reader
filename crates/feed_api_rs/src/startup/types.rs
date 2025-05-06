#[derive(Clone, Copy)]
pub enum Status {
    UnLaunch,
    Running,
    Completed,
    Aborted,
    Error,
}

#[derive(Clone, Copy)]
pub struct TaskDump {
    pub status: Status,
    pub duration: u128,
}
