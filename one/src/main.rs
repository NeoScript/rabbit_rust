use async_trait::async_trait;

use amqprs::{
    callbacks,
    channel::{BasicAckArguments, BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use tokio::time;

use tokio::sync::Notify;

struct Consumer {}

#[async_trait]
impl AsyncConsumer for Consumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let string = String::from_utf8(content).unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let result = channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let args = OpenConnectionArguments::new("localhost", 5672, "guest", "guest");
    let connection = Connection::open(&args).await.unwrap();

    connection
        .register_callback(callbacks::DefaultConnectionCallback)
        .await
        .unwrap();

    // Open an AMQP channel on this connection.
    let channel = connection.open_channel(None).await.unwrap();
    // Register channel level callbacks.
    // In production, user should create its own type and implement trait `ChannelCallback`.
    channel
        .register_callback(callbacks::DefaultChannelCallback)
        .await
        .unwrap();

    channel.queue_declare(QueueDeclareArguments::new("test"))
        .await
        .unwrap();

    channel
        .queue_bind(QueueBindArguments::new("test", "amq.topic", "*"))
        .await
        .unwrap();

    let queue_name = "test";
    let args = BasicConsumeArguments::new(&queue_name, "consumer_tag");

    let consumer = Consumer {};
    channel.basic_consume(consumer, args).await.unwrap();

    let guard = Notify::new();
    guard.notified().await;
}
