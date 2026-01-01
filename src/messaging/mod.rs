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
pub struct ProduceSuccess {}

#[derive(Debug)]
pub struct ProduceFailure {}

pub trait Producer {
    fn publish(
        &self,
        m: &Message,
    ) -> impl std::future::Future<Output = Result<ProduceSuccess, ProduceFailure>> + Send;
}

#[derive(Debug)]
pub struct ConsumeSuccess {}

#[derive(Debug)]
pub struct ConsumeFailure {}

#[derive(Debug)]
pub struct RunSuccess {}

#[derive(Debug)]
pub struct RunFailure {}

#[async_trait]
pub trait Consumer {
    async fn consume(&self, channel: String) -> Result<ConsumeSuccess, ConsumeFailure>;
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
