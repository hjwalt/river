use std::collections::HashMap;

use async_trait::async_trait;

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

#[derive(Debug)]
pub struct RunSuccess {}

#[derive(Debug)]
pub struct RunFailure {}

#[async_trait]
pub trait Subscriber {
    async fn subscribe(&self, channel: String) -> Result<SubscribeSuccess, SubscribeFailure>;

    async fn run(&self) -> Result<RunSuccess, RunFailure>;

    fn stop(&self);
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
