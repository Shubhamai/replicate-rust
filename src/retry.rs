//! Helper struct for the prediction struct. Used to retry pooling the api for latest prediction status until it is completed.

/// Strategy to use for retrying. Currently only fixed delay is supported.
pub enum RetryStrategy {
    // Retry with a fixed delay.
    FixedDelay(u64),
    // Retry with an exponential backoff.
    // ExponentialBackoff(u32),
}

/// TODO : Unimplemented
pub struct RetryPolicy {
    pub max_retries: u32,
    pub strategy: RetryStrategy,
    // step: u32,
}

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
