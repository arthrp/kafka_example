use std::time::Duration;

use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting");
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("client.id", "my_producer")
        .create()?;

    let record = FutureRecord::to("my_topic")
    .payload("Hello, kafka!")
    .key("sample_key");

    let delivery_status = producer.send(record, Duration::from_secs(1)).await;

    match delivery_status {
        Ok(metadata) => println!(
            "Message delivered. Partion: {}, Offset: {}", metadata.0, metadata.1
        ),
        Err((e, _)) => println!("Delivery failed: {}", e),
    }

    println!("Finished");

    Ok(())
}