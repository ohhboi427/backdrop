pub mod me;
pub mod photo;
pub mod search;
pub mod user;
pub mod topic;

pub use http::Method;

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub trait Endpoint: Serialize {
    type Response: DeserializeOwned;

    const METHOD: Method;

    fn path(&self) -> Cow<'static, str>;
}
