use amiquip::{AmqpProperties, Channel, Delivery, Exchange, Publish};

const NON_PERSISTENT_MESSAGE: u8 = 1;

pub async fn rpc(
    delivery: &Delivery,
    channel: &Channel,
    body: &[u8],
) -> Result<&'static str, &'static str> {
    let exchange = Exchange::direct(&channel);
    let (reply_to, corr_id) = match (
        delivery.properties.reply_to(),
        delivery.properties.correlation_id(),
    ) {
        (Some(r), Some(c)) => (r.clone(), c.clone()),
        _ => return Err("Received delivery without reply_to or correlation_id"),
    };
    exchange
        .publish(Publish::with_properties(
            &body,
            reply_to,
            AmqpProperties::default()
                .with_correlation_id(corr_id)
                .with_delivery_mode(NON_PERSISTENT_MESSAGE),
        ))
        .unwrap();
    Ok("Data published to reply-to queue")
}
