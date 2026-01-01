use std::{collections::HashMap, sync::atomic::AtomicBool};

use async_trait::async_trait;
use rdkafka::{
    ClientConfig, ClientContext, Message, TopicPartitionList,
    consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::{KafkaError, KafkaResult},
};

use crate::{
    core::{InitFailure, InitSuccess, RunFailure, RunSuccess, Service, StopFailure, StopSuccess},
    messaging::{ConsumeFailure, ConsumeSuccess},
};

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

type KafkaStreamConsumer = StreamConsumer<KafkaContext>;

pub struct KafkaConsumer {
    consumer: KafkaStreamConsumer,
    running: AtomicBool,
}

#[derive(Debug)]
pub struct KafkaConsumerError {}

pub async fn new(
    configuration: HashMap<String, String>,
) -> Result<KafkaConsumer, KafkaConsumerError> {
    let mut config = ClientConfig::new();
    for (key, value) in &configuration {
        config.set(key, value);
    }

    let context = KafkaContext;
    let create_status: Result<KafkaStreamConsumer, KafkaError> =
        config.clone().create_with_context(context);
    if create_status.is_err() {
        return Err(KafkaConsumerError {});
    }

    let consumer: KafkaStreamConsumer = create_status.ok().unwrap();
    let subscriber = KafkaConsumer {
        consumer: consumer,
        running: AtomicBool::new(false),
    };

    Ok(subscriber)
}

#[async_trait]
impl crate::messaging::Consumer for KafkaConsumer {
    async fn consume(&self, channel: String) -> Result<ConsumeSuccess, ConsumeFailure> {
        let topics = [channel.as_str()];
        self.consumer
            .subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        Ok(ConsumeSuccess {})
    }
}

// note that this large loop is fine as long as there are "await", otherwise use tokio::task::yield_now()
#[async_trait]
impl Service for KafkaConsumer {
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

    async fn init(&self) -> Result<InitSuccess, InitFailure> {
        Ok(InitSuccess {})
    }

    async fn stop(&self) -> Result<StopSuccess, StopFailure> {
        self.consumer.unsubscribe();
        Ok(StopSuccess {})
    }
}
