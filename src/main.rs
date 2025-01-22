mod async_producer;
mod config;
mod repeat_times;

use crate::async_producer::produce;
use crate::config::setup_logger;
use clap::{arg, Parser};
use log::info;
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

    /// message to publish (can be multiple)
    #[arg(short, long)]
    message: Vec<String>,

    /// message key (can be multiple)
    #[arg(short, long)]
    key: Vec<String>,

    /// Number of messages to publish
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Number of repeate or infinite
    #[arg(short, long, default_value_t = String::from("1"))]
    repeat: String,

    /// Delay between repeats in ms
    #[arg(short, long, default_value_t = 0)]
    delay: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logger(true, Option::Some("rdkafka=info"));
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    let repeat_times = if args.repeat == String::from("infinite") || args.repeat == "i" {
        repeat_times::RepeatTimes::Infinite
    } else {
        repeat_times::RepeatTimes::Times(args.repeat.parse::<usize>().unwrap())
    };

    produce(
        &args.brokers,
        &args.topic,
        &args.message,
        &args.key,
        args.count,
        repeat_times,
        args.delay,
    )
    .await;
}
