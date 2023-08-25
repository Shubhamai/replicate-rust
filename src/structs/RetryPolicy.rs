use crate::enums::RetryStrategy::RetryStrategy;

pub struct RetryPolicy {
    pub max_retries: u32,
    pub strategy: RetryStrategy,
    // step: u32,
}
