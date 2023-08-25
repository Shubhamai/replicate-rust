use crate::{enums::RetryStrategy::RetryStrategy, structs::RetryPolicy::RetryPolicy};

impl RetryPolicy {
    pub fn new(max_retries: u32, strategy: RetryStrategy) -> Self {
        Self {
            max_retries,
            strategy,
            // step: 0,
        }
    }

    pub fn step(&self) {
        match self.strategy {
            RetryStrategy::FixedDelay(delay) => {
                std::thread::sleep(std::time::Duration::from_millis(delay))
            } // RetryStrategy::ExponentialBackoff(delay) => delay * attempt,
        }
    }
}
