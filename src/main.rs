mod async_producer;
mod config;
mod producer;

use log::info;

use crate::async_producer::produce;
use crate::config::setup_logger;
use clap::Parser;
use rdkafka::util::get_rdkafka_version;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Kafka brokers address
    #[arg(short, long, default_value_t = String::from("localhost:9092"))]
    brokers: String,

    /// topic to publish message
    #[arg(short, long)]
    topic: String,

    /// message to publish
    #[arg(short, long)]
    message: String,

    /// message key
    #[arg(short, long, default_value_t = String::from(""))]
    key: String,

    /// Number of messages to publish
    #[arg(short, long, default_value_t = 1)]
    count: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logger(true, Option::Some("rdkafka=info"));
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    produce(
        &args.brokers,
        &args.topic,
        &args.message,
        &args.key,
        args.count,
    )
    .await;
}
