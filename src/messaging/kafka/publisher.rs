use std::{collections::HashMap, time::Duration};

use rdkafka::{
    ClientConfig,
    error::KafkaError,
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};

use crate::messaging::{Message, PublishFailure, PublishSuccess, Publisher};

pub struct KafkaPublisher {
    producer: FutureProducer,
}

#[derive(Debug)]
pub struct KafkaPublisherError {}

pub async fn new(
    configuration: HashMap<String, String>,
) -> Result<KafkaPublisher, KafkaPublisherError> {
    let mut config = ClientConfig::new();
    for (key, value) in &configuration {
        config.set(key, value);
    }

    let create_status: Result<FutureProducer, KafkaError> = config.clone().create();
    if create_status.is_err() {
        return Err(KafkaPublisherError {});
    }

    let producer: FutureProducer = create_status.ok().unwrap();

    Ok(KafkaPublisher { producer: producer })
}

impl Publisher for KafkaPublisher {
    async fn publish(&self, m: &Message) -> Result<PublishSuccess, PublishFailure> {
        let mut headers = OwnedHeaders::new();
        for (key, values) in &m.metadata {
            for value in values {
                headers = headers.insert(Header {
                    key: key,
                    value: Some(value),
                });
            }
        }

        let produce_status = self
            .producer
            .send(
                FutureRecord::to(m.channel.as_str())
                    .key(&m.key)
                    .payload(&m.body)
                    .headers(headers),
                Duration::from_secs(0),
            )
            .await;

        if produce_status.is_err() {
            return Err(PublishFailure {});
        }

        Ok(PublishSuccess {})
    }
}
