use serde::Serialize;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

#[derive(Debug, Default)]
pub struct Metrics {
    pub total_requests: AtomicU64,
    pub allowed_requests: AtomicU64,
    pub rate_limited_requests: AtomicU64,
}

#[derive(Serialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub rate_limited_requests: u64,
}

#[derive(Clone, Debug, Default)]
pub struct MetricsState(pub Arc<Metrics>);

impl MetricsState {
    pub fn new() -> Self {
        Self(Arc::new(Metrics::default()))
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.0.total_requests.load(Ordering::Relaxed),
            allowed_requests: self.0.allowed_requests.load(Ordering::Relaxed),
            rate_limited_requests: self.0.rate_limited_requests.load(Ordering::Relaxed),
        }
    }

    pub fn inc_total(&self) {
        self.0.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_allowed(&self) {
        self.0.allowed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_rate_limited(&self) {
        self.0
            .rate_limited_requests
            .fetch_add(1, Ordering::Relaxed);
    }
}
