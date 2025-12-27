use std::{collections::HashMap, sync::Arc};

use rdkafka::{
    ClientConfig, ClientContext, TopicPartitionList,
    consumer::{BaseConsumer, ConsumerContext, Rebalance, StreamConsumer},
    error::{KafkaError, KafkaResult},
};

use crate::messaging::{self, SubscribeFailure, SubscribeSuccess, Subscriber};

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
    let subscriber = KafkaSubscriber { consumer: consumer };

    Ok(subscriber)
}

async fn subscribe_inner(kafka: Arc<&'static KafkaSubscriber>, channel: String) -> Result<(), ()> {
    // let shared_self = Arc::new(kafka);

    let local_self = Arc::clone(&kafka);
    let spawn_handle = tokio::spawn(async move {
        // let local_self = &self_copy;

        local_self.subscriber_loop().await

        // loop_var.await;
    });

    spawn_handle.await;

    Ok(())
}

impl Subscriber for KafkaSubscriber {
    async fn subscribe(
        &'static self,
        channel: String,
    ) -> Result<SubscribeSuccess, SubscribeFailure> {
        let shared_self = Arc::new(self);
        subscribe_inner(shared_self, channel);

        // {
        //     let local_self = Arc::clone(&shared_self);
        //     tokio::spawn({ local_self.subscriber_loop() });
        // }
        // let fun_name = async move {
        //     let local_self = &self_copy;

        //     let loop_var = local_self.subscriber_loop();

        //     loop_var.await;
        // };

        // let spawn_handle = tokio::spawn(async move {
        //     // let local_self = &self_copy;

        //     let loop_var = local_self.subscriber_loop();

        //     loop_var.await;
        // });

        Ok(SubscribeSuccess {})
    }
}

impl KafkaSubscriber {
    async fn subscriber_loop(&self) -> Result<(), ()> {
        Ok(())
    }
}
