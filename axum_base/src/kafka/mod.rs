use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
    message::{OwnedHeaders, Header},
    util::Timeout,
};
use std::time::Duration;
use tracing::info;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Self {
            producer,
            topic: topic.to_string(),
        }
    }

    /// 发送单条消息
    pub async fn send_message(
        &self,
        key: &str,
        payload: &str,
        headers: Option<OwnedHeaders>,
    ) -> Result<(), String> {
        let record = FutureRecord::to(&self.topic)
            .payload(payload)
            .key(key);

        let record = if let Some(headers) = headers {
            record.headers(headers)
        } else {
            record
        };

        self.producer
            .send(record, Timeout::After(Duration::from_secs(0)))
            .await
            .map(|_| ())
            .map_err(|(err, _)| format!("Failed to send message: {}", err))
    }

    /// 批量发送消息
    pub async fn send_messages<'a, I>(&self, messages: I) -> Vec<Result<(), String>>
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let futures = messages
            .into_iter()
            .map(|(key, payload)| async move {
                let delivery_status = self
                    .send_message(
                        key,
                        payload,
                        Some(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    )
                    .await;

                info!("Delivery status for message with key {} received", key);
                delivery_status
            })
            .collect::<Vec<_>>();

        let mut results = Vec::new();
        for future in futures {
            let result = future.await;
            info!("Future completed. Result: {:?}", result);
            results.push(result);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kafka_producer() {
        let producer = KafkaProducer::new("localhost:9092", "test-topic");
        
        // 测试发送单条消息
        let result = producer
            .send_message(
                "test-key",
                "test-message",
                Some(OwnedHeaders::new().insert(Header {
                    key: "test-header",
                    value: Some("test-value"),
                })),
            )
            .await;
        assert!(result.is_ok());

        // 测试批量发送消息
        let messages = vec![
            ("key1", "message1"),
            ("key2", "message2"),
            ("key3", "message3"),
        ];
        let results = producer.send_messages(messages).await;
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
