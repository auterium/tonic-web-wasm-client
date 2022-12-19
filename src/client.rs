use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Future;
use http::{Request, Response};
use tonic::{body::BoxBody, client::GrpcService};

use crate::{call::call, Error, ResponseBody};

/// `grpc-web` based transport layer for `tonic` clients
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    force_gzip: bool,
}

impl Client {
    /// Creates a new client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            force_gzip: false,
        }
    }

    /// Creates a new client that forces gzip header
    pub fn new_with_gzip_forced(base_url: String) -> Self {
        Self {
            base_url,
            force_gzip: true,
        }
    }
}

impl GrpcService<BoxBody> for Client {
    type ResponseBody = ResponseBody;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<BoxBody>) -> Self::Future {
        Box::pin(call(self.base_url.clone(), self.force_gzip, request))
    }
}
