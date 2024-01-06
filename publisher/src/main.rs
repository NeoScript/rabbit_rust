use amqprs::{
    callbacks,
    channel::BasicPublishArguments,
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};

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

    for i in 0..1_000_000 {
        let content = format!("message number {}", i).into_bytes();
        channel
            .basic_publish(
                BasicProperties::default(),
                content,
                BasicPublishArguments::new("amq.topic", "test"),
            )
            .await
            .unwrap();
    }
}
