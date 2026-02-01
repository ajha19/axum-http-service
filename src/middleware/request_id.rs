use axum::{
    body::Body,
    http::{Request, Response},
    // response::IntoResponse, // Removed
};
use futures_util::future::BoxFuture;
use http::{HeaderName, HeaderValue};
use std::{
    task::{Context, Poll},
    // time::Duration, // Removed
};
use tower::{Layer, Service};
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RequestIdMiddleware<S>
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

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let header_name = HeaderName::from_static("x-request-id");
        let header_value = HeaderValue::from_str(&request_id).unwrap();

        req.headers_mut().insert(header_name.clone(), header_value.clone());

        let future = self.inner.call(req);

        Box::pin(async move {
            let mut response = future.await?;
            response.headers_mut().insert(header_name, header_value);
            Ok(response)
        })
    }
}
