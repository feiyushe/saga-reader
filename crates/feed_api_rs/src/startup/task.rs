use std::future::Future;

use spdlog::{error, info};
use tokio::time::Instant;

use super::types::{Status, TaskDump};

pub trait TaskInitializer<TResult, TData> {
    fn start<F: FnOnce() -> Fut, Fut: Future<Output = anyhow::Result<TData>>>(
        &mut self,
        task_name: &str,
        function: F,
    ) -> impl Future<Output = anyhow::Result<()>>;

    fn dump(&self) -> TaskDump;
}

pub struct InitTask<TData> {
    pub result: Option<TData>,
    pub task_cost: u128,
    pub task_status: Status,
}

impl<TData> Default for InitTask<TData> {
    fn default() -> Self {
        InitTask {
            result: None,
            task_cost: 0,
            task_status: Status::UnLaunch,
        }
    }
}

impl<TData> TaskInitializer<InitTask<TData>, TData> for InitTask<TData> {
    async fn start<F: FnOnce() -> Fut, Fut: Future<Output = anyhow::Result<TData>>>(
        &mut self,
        task_name: &str,
        function: F,
    ) -> anyhow::Result<()> {
        let duration_elapse = Instant::now();
        self.task_status = Status::Running;
        match function().await {
            Ok(result) => {
                self.result = Some(result);
                self.task_status = Status::Completed;
                self.task_cost = duration_elapse.elapsed().as_millis();
                info!("the task {} execute completed, cost {}ms", task_name, self.task_cost);
                Ok(())
            }
            Err(err) => {
                self.task_status = Status::Error;
                error!("the task {} execute error...{}", task_name, err);
                Err(err)
            }
        }
    }

    fn dump(&self) -> super::types::TaskDump {
        TaskDump {
            status: self.task_status,
            duration: self.task_cost,
        }
    }
}
