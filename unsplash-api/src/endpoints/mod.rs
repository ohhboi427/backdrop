pub mod users;
pub mod photos;

use crate::client::Client;
use http::Method;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub trait Endpoint {
    type Output: DeserializeOwned;

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str>;
}

pub trait Query {
    type Output: DeserializeOwned;

    fn query(&self, client: &Client) -> impl Future<Output = Self::Output>;
}

impl<E: Endpoint> Query for E {
    type Output = E::Output;

    fn query(&self, client: &Client) -> impl Future<Output = Self::Output> {
        client.execute(self.method(), self.endpoint())
    }
}
