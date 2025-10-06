mod async_producer;
mod config;
mod file_utils;
mod repeat_times;

use crate::config::setup_logger;
use crate::{async_producer::produce, file_utils::read_lines};
use clap::{arg, Parser};
use log::{error, info};
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

    /// File containing messages (one per line)
    #[arg(short = 'f', long = "file")]
    file: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut args = Args::parse();

    setup_logger(true, Option::Some("rdkafka=info"));
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{version_n:08x}, {version_s}");
    let repeat_times = if args.repeat == "infinite" || args.repeat == "i" {
        repeat_times::RepeatTimes::Infinite
    } else {
        repeat_times::RepeatTimes::Times(args.repeat.parse::<usize>().unwrap())
    };

    // If file is provided, read messages from file
    if let Some(ref file_path) = args.file {
        match read_lines(file_path) {
            Ok(lines) => {
                args.message = lines;
            }
            Err(e) => {
                error!("Error reading message file: {e}");
                std::process::exit(1);
            }
        }
    }

    info!("arg {args:?}");
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
