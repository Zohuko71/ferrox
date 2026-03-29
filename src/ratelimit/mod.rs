pub mod token_bucket;

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::VirtualKeyConfig;
use token_bucket::TokenBucket;

pub type RateLimiter = HashMap<String, Arc<TokenBucket>>;

/// Build one token bucket per virtual key that has a `rate_limit` config.
/// Keys without a `rate_limit` entry are not tracked (unlimited).
pub fn build_rate_limiter(virtual_keys: &[VirtualKeyConfig]) -> RateLimiter {
    let mut map = HashMap::new();
    for key in virtual_keys {
        if let Some(rl) = &key.rate_limit {
            map.insert(
                key.name.clone(),
                Arc::new(TokenBucket::new(rl.requests_per_minute, rl.burst)),
            );
        }
    }
    map
}
