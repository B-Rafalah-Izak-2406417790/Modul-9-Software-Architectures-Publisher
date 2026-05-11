use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use std::error::Error;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::connect(
        "amqp://guest:guest@127.0.0.1:5672/%2f",
        ConnectionProperties::default(),
    )
    .await?;
    let channel = connection.create_channel().await?;

    publish_event(
        &channel,
        "user_created",
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406417790-Amir".to_owned(),
        },
    )
    .await?;
    publish_event(
        &channel,
        "user_created",
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406417790-Budi".to_owned(),
        },
    )
    .await?;
    publish_event(
        &channel,
        "user_created",
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406417790-Cica".to_owned(),
        },
    )
    .await?;
    publish_event(
        &channel,
        "user_created",
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406417790-Dira".to_owned(),
        },
    )
    .await?;
    publish_event(
        &channel,
        "user_created",
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406417790-Emir".to_owned(),
        },
    )
    .await?;

    connection.close(0, "publisher finished").await?;
    Ok(())
}

async fn publish_event(
    channel: &Channel,
    event_name: &str,
    message: UserCreatedEventMessage,
) -> Result<(), Box<dyn Error>> {
    let exchange_name = format!("{event_name}_exchange");
    let mut buffer = Vec::new();
    message.serialize(&mut buffer)?;

    channel
        .exchange_declare(
            &exchange_name,
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .basic_publish(
            &exchange_name,
            event_name,
            BasicPublishOptions::default(),
            &buffer,
            BasicProperties::default(),
        )
        .await?
        .await?;

    Ok(())
}
