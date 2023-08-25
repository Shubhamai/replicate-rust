pub enum RetryStrategy {
    // Retry with a fixed delay.
    FixedDelay(u64),
    // Retry with an exponential backoff.
    // ExponentialBackoff(u32),
}
