use std::time::Duration;
use std::time::Instant;

use log::debug;
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[allow(dead_code)]
pub async fn produce(
    brokers: &str,
    topic_name: &str,
    message: &str,
    key: &str,
    message_count: i32,
) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    let before = Instant::now();

    let futures = (0..message_count)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name).payload(message).key(key),
                    Duration::from_secs(0),
                )
                .await;

            debug!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        let _ = future.await;
    }
    info!(
        "finished publishing {} events in {:?} ",
        message_count,
        before.elapsed()
    );
}
