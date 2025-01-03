use log::info;
use std::time::Duration;
use std::time::Instant;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{
    BaseRecord, DefaultProducerContext, NoCustomPartitioner, Producer, ThreadedProducer,
};

pub async fn produce(
    brokers: &str,
    topic_name: &str,
    message: &str,
    key: &str,
    message_count: i32,
) {
    let producer: &ThreadedProducer<DefaultProducerContext, NoCustomPartitioner> =
        &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

    for i in 0..message_count {
        let _ = producer.send(BaseRecord::to(topic_name).payload(message).key(key));
    }
    producer.flush(Duration::from_secs(10)).unwrap();

    let before = Instant::now();

    info!(
        "finished publishing {} events in {:?} ",
        message_count,
        before.elapsed()
    );
}
