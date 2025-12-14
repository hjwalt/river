use std::collections::HashMap;

use bytes::Buf;

pub mod kafka;

pub struct Message {
    key: String,
    body: Vec<u8>,
    metadata: HashMap<String, Vec<String>>,
}

pub struct TypedMessage<T> {
    key: String,
    body: T,
    metadata: HashMap<String, Vec<String>>,
}

pub struct PublishSuccess {}

pub struct PublishFailure {}

pub trait Publisher {
    async fn publish(&self, m: Message) -> Result<PublishSuccess, PublishFailure>;
}

pub struct SubscribeSuccess {}

pub struct SubscribeFailure {}

pub trait Subscriber {
    async fn subscribe(&self) -> Result<SubscribeSuccess, SubscribeFailure>;
}

pub struct HandleSuccess {}

pub struct HandleFailure {}

pub trait Handler {
    async fn handle(&self, msg: Message) -> Result<HandleSuccess, HandleFailure>;
}
