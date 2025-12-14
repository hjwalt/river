use std::time::Duration;

use rdkafka::{
    ClientConfig,
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};
use std_logger::Config;

#[tokio::main]
async fn main() -> Result<(), ()> {
    Config::logfmt().init();

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let topic_name = "test".to_string();

    let headers = OwnedHeaders::new().insert(Header {
        key: "header_key",
        value: Some("header_value"),
    });

    let status = producer
        .send(
            FutureRecord::to(&topic_name)
                .payload("123456".as_bytes())
                .key("123456".as_bytes())
                .headers(headers),
            Duration::from_secs(0),
        )
        .await;

    log::info!("produced to cluster");
    Ok(())
}
