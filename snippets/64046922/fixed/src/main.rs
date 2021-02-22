use lapin::{
    options::*, publisher_confirm::{Confirmation, PublisherConfirm}, 
    types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Error
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

struct Topic {
    message: String,
    name: String,
    routing_key: String,
    publish_options: BasicPublishOptions
}


struct RabbitMQ {
    connection: Connection,
}

struct ConnectionDetails {
    uri: String
}

impl RabbitMQ {
    async fn connect(details: &ConnectionDetails) -> Self {
        RabbitMQ {
            connection: Connection::connect(
                &details.uri,
                ConnectionProperties::default().with_default_executor(8)
            ).await.unwrap()
        }
    }
}

#[async_trait]
trait MessageBroker {
    async fn publish(&self, topic: &Topic) -> Result<PublisherConfirm, Error>;
    async fn publish_all(&self, topic: &'static Topic);
}

#[async_trait]
impl MessageBroker for RabbitMQ {
    async fn publish(&self, topic: &Topic) -> Result<PublisherConfirm, Error> {
        let channel = self.connection.create_channel().await.unwrap();
        let payload = topic.message.as_bytes();
        channel.basic_publish(
            topic.name.as_str(),
            topic.routing_key.as_str(),
            topic.publish_options,
            payload.to_vec(),
            BasicProperties::default(),
        ).await
    }

    async fn publish_all(&self, my_topic: &'static Topic) {
        let connection_details = ConnectionDetails {uri: String::from("http://")};
        let broker = Arc::new(RabbitMQ::connect(&connection_details).await);
        for _ in 1..10 {
            let broker = Arc::clone(&broker);
            tokio::spawn(async move {
                let confirm = broker.publish(&my_topic).await.unwrap();
            }).await.unwrap();
        }
    }
}


fn main() {}
