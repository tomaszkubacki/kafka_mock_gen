use log::error;
use log::info;
use rdkafka::types::RDKafkaErrorCode;
use std::thread;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use std::time::Instant;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{
    BaseRecord, DefaultProducerContext, NoCustomPartitioner, Producer, ThreadedProducer,
};

use crate::repeat_times::RepeatTimes;

const FLUSH_TIME_SECS: u64 = 30;

pub async fn produce(
    brokers: &str,
    topic_name: &str,
    messages: &Vec<String>,
    keys: &Vec<String>,
    message_count: usize,
    repeat_times: crate::repeat_times::RepeatTimes,
    delay: u64,
) {
    let producer: &ThreadedProducer<DefaultProducerContext, NoCustomPartitioner> =
        &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "15000")
            .create()
            .expect("Producer creation error");

    info!("publishing {:?} messages", message_count);
    let keys_len = keys.len();
    let messages_len = messages.len();
    let mut repeat_count = 0;

    loop {
        let before = Instant::now();
        for n in 1..=message_count {
            loop {
                let key = &keys[n % keys_len];
                let message = &messages[n % messages_len];

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

        let do_break = match repeat_times {
            RepeatTimes::Infinite => false,
            RepeatTimes::Times(n) => {
                repeat_count += 1;
                n <= repeat_count
            }
        };

        if do_break {
            break;
        } else if delay > 0 {
            sleep(Duration::from_millis(delay));
        }
    }
}
