use rdkafka::{
    ClientConfig, ClientContext, Message, TopicPartitionList,
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::KafkaResult,
};
use std_logger::Config;

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

#[tokio::main]
async fn main() -> Result<(), ()> {
    Config::logfmt().init();

    let context = KafkaContext;

    let mut config = ClientConfig::new();

    config
        .set("group.id", "test-rs-2")
        .set("bootstrap.servers", "127.0.0.1:9092")
        // .set("enable.partition.eof", "false")
        // .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        //.set("statistics.interval.ms", "30000")
        .set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug);

    let consumer: KafkaConsumer = config
        .create_with_context(context)
        .expect("Consumer creation failed");

    log::info!("subscribing");

    let topics = ["test"];
    consumer
        .subscribe(&topics)
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
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
                consumer.commit_message(&m, CommitMode::Sync).unwrap();
            }
        };
    }
    Ok(())
}
