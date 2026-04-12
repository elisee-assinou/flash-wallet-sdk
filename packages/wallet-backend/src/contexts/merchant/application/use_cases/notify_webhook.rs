use reqwest::Client;
use serde_json::json;

pub struct WebhookNotifier {
    client: Client,
}

impl WebhookNotifier {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn notify(
        &self,
        webhook_url: &str,
        event: &str,
        invoice: &str,
        amount_sats: u64,
        description: &str,
        merchant: &str,
    ) {
        let payload = json!({
            "event": event,
            "invoice": invoice,
            "amount_sats": amount_sats,
            "description": description,
            "merchant": merchant,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        match self.client
            .post(webhook_url)
            .header("Content-Type", "application/json")
            .header("X-Flash-Webhook", "1")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => tracing::info!(
                " Webhook sent to {} → status: {}",
                webhook_url,
                resp.status()
            ),
            Err(e) => tracing::error!(
                " Webhook failed for {}: {}",
                webhook_url,
                e
            ),
        }
    }
}
