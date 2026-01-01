use std::{collections::HashMap, sync::Arc};

use river::{core::Service, messaging::Consumer};
use std_logger::Config;
use tokio::task::JoinHandle;

// careful with the thread count because if there are 8 long running loops, everything will be stuck
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), ()> {
    Config::logfmt().init();

    let subscriber = river::messaging::kafka::consumer::new(HashMap::from([
        ("group.id".to_owned(), "test-rs-2".to_owned()),
        ("bootstrap.servers".to_owned(), "127.0.0.1:9092".to_owned()),
        ("enable.auto.commit".to_owned(), "false".to_owned()),
        ("auto.offset.reset".to_owned(), "smallest".to_owned()),
    ]))
    .await
    .expect("failed to create subscriber");

    log::info!("subscribing");

    subscriber
        .consume("test".to_owned())
        .await
        .expect("failed to subscribe to topic");

    let subscriber_arc = Arc::new(subscriber);
    let subscriber_arc_self = subscriber_arc.clone();

    // subscriber_arc is now owned by this long running loop
    let handle: JoinHandle<Result<(), ()>> = tokio::spawn(async move {
        subscriber_arc.run().await.expect("ok");
        Ok(())
    });

    log::info!("subscriber started");

    // subscriber_arc_self.stop().await;

    handle.await.expect("ok");
    // subscriber_arc.stop();

    Ok(())
}
