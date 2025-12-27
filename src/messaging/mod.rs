use std::collections::HashMap;

use tokio::task::JoinHandle;

pub mod kafka;

#[derive(Debug)]
pub struct Message {
    pub channel: String,
    pub key: String,
    pub body: Vec<u8>,
    pub metadata: HashMap<String, Vec<String>>,
}

pub struct TypedMessage<T> {
    pub channel: String,
    pub key: String,
    pub body: T,
    pub metadata: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct PublishSuccess {}

#[derive(Debug)]
pub struct PublishFailure {}

pub trait Publisher {
    fn publish(
        &self,
        m: &Message,
    ) -> impl std::future::Future<Output = Result<PublishSuccess, PublishFailure>> + Send;
}

#[derive(Debug)]
pub struct SubscribeSuccess {}

#[derive(Debug)]
pub struct SubscribeFailure {}

pub trait Subscriber {
    fn subscribe(
        &'static self,
        channel: String,
    ) -> impl std::future::Future<Output = Result<SubscribeSuccess, SubscribeFailure>> + Send;
}

#[derive(Debug)]
pub struct HandleSuccess {}

#[derive(Debug)]
pub struct HandleFailure {}

pub trait Handler {
    fn handle(
        &self,
        msg: Message,
    ) -> impl std::future::Future<Output = Result<HandleSuccess, HandleFailure>> + Send;
}
