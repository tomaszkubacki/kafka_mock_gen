# Kafka mock gen

Project goal is to create fast and easy to use message producer for kafka

## prerequisites

1) Kafka kluster
Fast and easy way is to run it in docker e.g. use my 
[docker-compose kafka docker dev repository](https://github.com/tomaszkubacki/kafka_docker_dev) 

2) librdkafka-dev

e.g. on Debian/Ubuntu install with

sudo apt install librdkafka-dev

## usage
```
cargo run -- -b localhost:9092 -t topic-1 -m '{"a": 2}' -c 200
```

