# Kafka mock gen

Project goal is to create fast and easy to use message producer for kafka

## prerequisites

1) Kafka kluster
Fast and easy way is to run it in a docker e.g. use my 
[docker-compose kafka docker dev repository](https://github.com/tomaszkubacki/kafka_docker_dev) 

2) librdkafka-dev

e.g. on Debian/Ubuntu install with

sudo apt install librdkafka-dev

### cross compile to windows
install prerequisites
```
sudo apt-get install cmake mingw-w64
```

do compile

```
cargo build --target x86_64-pc-windows-gnu -r
```

## usage

display all usage options with --help flag
```
cargo run -- --help
```

or invoke directly with

```
cargo run -- -b localhost:9092 -t topic-1 -m '{"a": 2}' -k "1234567890" -c 200
```

