use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicBool},
};

use async_trait::async_trait;
use rdkafka::{
    ClientConfig, ClientContext, Message, TopicPartitionList,
    consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::{KafkaError, KafkaResult},
};

use crate::messaging::{RunFailure, RunSuccess, SubscribeFailure, SubscribeSuccess, Subscriber};

struct KafkaContext;

impl ClientContext for KafkaContext {}

impl ConsumerContext for KafkaContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        log::info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        log::info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        log::info!("Committing offsets: {:?}", result);
    }
}

type KafkaConsumer = StreamConsumer<KafkaContext>;

pub struct KafkaSubscriber {
    consumer: KafkaConsumer,
    running: AtomicBool,
}

#[derive(Debug)]
pub struct KafkaSubscriberError {}

pub async fn new(
    configuration: HashMap<String, String>,
) -> Result<KafkaSubscriber, KafkaSubscriberError> {
    let mut config = ClientConfig::new();
    for (key, value) in &configuration {
        config.set(key, value);
    }

    let context = KafkaContext;
    let create_status: Result<KafkaConsumer, KafkaError> =
        config.clone().create_with_context(context);
    if create_status.is_err() {
        return Err(KafkaSubscriberError {});
    }

    let consumer: KafkaConsumer = create_status.ok().unwrap();
    let subscriber = KafkaSubscriber {
        consumer: consumer,
        running: AtomicBool::new(false),
    };

    Ok(subscriber)
}

#[async_trait]
impl Subscriber for KafkaSubscriber {
    async fn subscribe(&self, channel: String) -> Result<SubscribeSuccess, SubscribeFailure> {
        let topics = [channel.as_str()];
        self.consumer
            .subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        Ok(SubscribeSuccess {})
    }

    async fn run(&self) -> Result<RunSuccess, RunFailure> {
        // self.running.store(false, order);
        // self.running.
        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    log::warn!("Kafka error: {}", e);
                    break;
                }
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            log::warn!("Error while deserializing message payload: {:?}", e);
                            break;
                        }
                    };
                    log::info!(
                        "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                        m.key(),
                        payload,
                        m.topic(),
                        m.partition(),
                        m.offset(),
                        m.timestamp()
                    );
                    // if let Some(headers) = m.headers() {
                    //     for header in headers.iter() {
                    //         log::info!("  Header {:#?}: {:?}", header.key, header.value);
                    //     }
                    // }
                    self.consumer.commit_message(&m, CommitMode::Sync).unwrap();
                }
            };
        }

        log::info!("exit");
        Ok(RunSuccess {})
    }

    fn stop(&self) {
        // self.running = false;
    }
}
