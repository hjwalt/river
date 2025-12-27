use std::collections::HashMap;

use river::messaging::{Message, Publisher};
use std_logger::Config;

#[tokio::main]
async fn main() -> Result<(), ()> {
    Config::logfmt().init();

    let publisher = river::messaging::kafka::publisher::new(HashMap::from([
        ("bootstrap.servers".to_owned(), "127.0.0.1:9092".to_owned()),
        ("message.timeout.ms".to_owned(), "5000".to_owned()),
    ]))
    .await
    .expect("failed to create publisher");

    let topic_name = "test".to_string();

    let message = Message {
        channel: topic_name.to_owned(),
        key: "123456".to_owned(),
        body: Vec::from("123456".as_bytes()),
        metadata: HashMap::from([("header_key".to_owned(), vec!["".to_owned()])]),
    };

    let status = publisher.publish(&message).await;

    if status.is_err() {
        return Err(());
    }
    log::info!("produced to cluster");

    Ok(())
}
