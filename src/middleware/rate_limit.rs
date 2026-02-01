use crate::metrics_state::MetricsState;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use futures_util::future::BoxFuture;
use std::{
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct RateLimitLayer {
    state: Arc<Mutex<RateLimitState>>,
    metrics: MetricsState,
    max_requests: u64,
    window: Duration,
}

impl RateLimitLayer {
    pub fn new(max_requests: u64, window: Duration, metrics: MetricsState) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimitState {
                requests: 0,
                window_start: Instant::now(),
            })),
            metrics,
            max_requests,
            window,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware {
            inner,
            state: self.state.clone(),
            metrics: self.metrics.clone(),
            max_requests: self.max_requests,
            window: self.window,
        }
    }
}

struct RateLimitState {
    requests: u64,
    window_start: Instant,
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    state: Arc<Mutex<RateLimitState>>,
    metrics: MetricsState,
    max_requests: u64,
    window: Duration,
}

impl<S> Service<Request<Body>> for RateLimitMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();

        if now.duration_since(state.window_start) >= self.window {
            state.requests = 0;
            state.window_start = now;
        }

        if state.requests >= self.max_requests {
            self.metrics.inc_rate_limited(); // Increment blocked counter
            let response = (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests").into_response();
            return Box::pin(async move { Ok(response) });
        }

        state.requests += 1;
        drop(state);

        let future = self.inner.call(req);
        Box::pin(future)
    }
}
