/// Retry configuration for transient failures (429, 503).
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub backoff_base_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 4,
            backoff_base_ms: 250,
        }
    }
}
