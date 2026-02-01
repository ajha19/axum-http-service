use crate::metrics_state::MetricsState;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::{
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct MetricsLayer {
    pub state: MetricsState,
}

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct MetricsMiddleware<S> {
    inner: S,
    state: MetricsState,
}

impl<S> Service<Request<Body>> for MetricsMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // 1. Increment total requests
        self.state.inc_total();

        let start = Instant::now();
        let path = req.uri().path().to_string();
        let method = req.method().clone();
        let state = self.state.clone();

        let future = self.inner.call(req);

        Box::pin(async move {
            let response = future.await?;
            let duration = start.elapsed();
            let status = response.status();

            // 2. Increment allowed requests if success (RateLimit layer handles 429 logic, 
            // but if we reach here and it was 429, it means the handler returned 429 
            // OR the rate limit layer returned 429. 
            // Actually, we need to coordinate with RateLimitedLayer.
            // If the Response is 429, we assume it was blocked? 
            // Or better: RateLimitLayer increments 'blocked', so here we increment 'allowed' only if it wasn't blocked?
            // Wait, if RateLimitLayer returns early, it returns a Response.
            // So THIS middleware sees the response.
            
            if status == axum::http::StatusCode::TOO_MANY_REQUESTS {
                // It was rate limited (most likely). 
                // However, the user requirement says:
                // "Handlers (allowed_requests++ on success)"
                // "Rate Limit (check/allow or block)"
                // If Rate Limit blocks, it returns 429 transparently.
                // So this middleware will see 429.
            } else {
                state.inc_allowed();
            }

            info!(
                method = %method,
                path = %path,
                status = %status,
                latency_ms = %duration.as_millis(),
                "request_completed"
            );

            Ok(response)
        })
    }
}
