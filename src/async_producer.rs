use log::error;
use log::info;
use rdkafka::types::RDKafkaErrorCode;
use std::thread;
use std::time;
use std::time::Duration;
use std::time::Instant;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{
    BaseRecord, DefaultProducerContext, NoCustomPartitioner, Producer, ThreadedProducer,
};

const FLUSH_TIME_SECS: u64 = 30;

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
            .set("message.timeout.ms", "15000")
            .create()
            .expect("Producer creation error");

    info!("publishing {:?} messages", message_count);
    let before = Instant::now();

    for _ in 0..message_count {
        loop {
            match producer.send(BaseRecord::to(topic_name).key(key).payload(message)) {
                Ok(_) => break,
                Err((err, _record)) => {
                    if err.rdkafka_error_code().unwrap() == RDKafkaErrorCode::QueueFull {
                        // warn!("Queue full, retrying...");
                        thread::sleep(time::Duration::from_millis(200));
                    } else {
                        error!("Failed to enqueue message: {:?}", err);
                        break; // Exit the loop on other errors
                    }
                }
            }
        }
    }
    producer
        .flush(Duration::from_secs(FLUSH_TIME_SECS))
        .unwrap();

    info!(
        "finished publishing {} events in {:?} ",
        message_count,
        before.elapsed()
    );
}
