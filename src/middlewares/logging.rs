use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{dev::Service, dev::ServiceRequest, dev::ServiceResponse, Error};
use std::task::{Context, Poll};
use futures_util::future::LocalBoxFuture;

#[derive(Debug)]
pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;


    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Modify the incoming request here (e.g., add custom headers)
        
        // Create a HeaderName from the string
        if let Ok(header_name) = HeaderName::from_bytes(b"X-Custom-Header") {
            // Create a HeaderValue from the string
            let header_value = HeaderValue::from_static("Custom Value");
            // Insert the header into the request
            req.headers_mut().insert(header_name, header_value);
        } 
        
        let fut = self.service.call(req);
        let fut = async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => Err(err.into()), // Convert any error to actix_web::Error
            }
        };

        Box::pin(fut)
    }
}
