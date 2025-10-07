use tokio::time::{sleep, Duration};
use std::time::SystemTime;

pub struct Scheduler {
    interval: Duration,
}

impl Scheduler {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }

    pub async fn run<F, Fut>(&self, mut task: F)
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        loop {
            task().await;
            sleep(self.interval).await;
        }
    }

    pub fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
