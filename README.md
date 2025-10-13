# Kafka mock gen

Project goal is to create fast and easy to use
mock message producer allowing stress kafka broker
e.g. publish 2M messages

```bash
cargo run -- -b localhost:9092 -t topic-1 -m '{"a": 2}' -k "1234567890" -c 2000000
```

## prerequisites

1. Kafka dev broker
   Fast and easy way is to run it in a docker e.g. use my
   [docker-compose kafka docker dev repository](https://github.com/tomaszkubacki/kafka_docker_dev)

2. librdkafka-dev
   e.g. on Debian/Ubuntu install with

   ```bash
   sudo apt install librdkafka-dev
   ```

## build release

```bash
cargo build -r
```

### cross compile to windows

Instal prerequisites

```bash
sudo apt-get install cmake mingw-w64
```

add windows as target build

```bash
rustup target add x86_64-pc-windows-gnu
```

Compile with target

```bash
cargo build --target x86_64-pc-windows-gnu -r
```

## usage

Displays all usage options with --help flag

```bash
cargo run -- --help
```

Invoke compiled version with

```shell
kafka-mock-gen -b localhost:9092 -t topic-1 -m '{"k": 9999}' -k "1234567890" -c 2000000
```
