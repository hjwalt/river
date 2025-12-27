use bytes::BytesMut;
use prost::Message;
use std_logger::Config;

#[tokio::main]
async fn main() -> Result<(), ()> {
    Config::logfmt().init();

    let shirt = river::domain::proto::Shirt::default();

    let mut buf = BytesMut::with_capacity(1024);

    let encoded = shirt.encode(&mut buf);
    if encoded.is_err() {
        return Err(());
    }

    let decoded = river::domain::proto::Shirt::decode(buf);

    log::info!("consuming {:?}", decoded);

    Ok(())
}
