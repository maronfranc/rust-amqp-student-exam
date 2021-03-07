use amiquip::{AmqpProperties, Channel, Delivery, Exchange, Publish};

const NON_PERSISTENT_MESSAGE: u8 = 1;

pub fn rpc(delivery: &Delivery, channel: &Channel, body: &[u8]) {
    let exchange = Exchange::direct(&channel);
    if let (Some(reply_to), Some(corr_id)) = (
        delivery.properties.reply_to(),
        delivery.properties.correlation_id(),
    ) {
        exchange
            .publish(Publish::with_properties(
                &body,
                reply_to,
                AmqpProperties::default()
                    .with_correlation_id(String::from(corr_id))
                    .with_delivery_mode(NON_PERSISTENT_MESSAGE),
            ))
            .unwrap();
    }
}
