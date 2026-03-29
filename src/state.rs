use std::sync::{atomic::AtomicBool, Arc};

use crate::config::Config;
use crate::metrics::Metrics;
use crate::providers::ProviderRegistry;
use crate::ratelimit::RateLimiter;
use crate::router::ModelRouter;

#[derive(Clone)]
#[allow(dead_code)] // metrics used in Phase 3 telemetry
pub struct AppState {
    pub config: Arc<Config>,
    pub providers: Arc<ProviderRegistry>,
    pub router: Arc<ModelRouter>,
    pub rate_limiter: Arc<RateLimiter>,
    pub metrics: Arc<Metrics>,
    pub ready: Arc<AtomicBool>,
}
