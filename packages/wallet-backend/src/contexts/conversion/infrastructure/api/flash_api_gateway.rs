use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::shared::errors::DomainError;
use crate::contexts::conversion::domain::{
    entities::flash_transaction::{FlashTransaction, TransactionType},
    value_objects::{
        xof_amount::XofAmount,
        momo_number::MomoNumber,
        satoshis::Satoshis,
    },
    ports::flash_gateway::FlashGateway,
};

#[derive(Serialize)]
struct CreateTransactionBody {
    amount: u64,
    receiver_address: String,
    #[serde(rename = "type")]
    transaction_type: String,
    number: String,
}

#[derive(Deserialize)]
struct CreateTransactionResponse {
    success: bool,
    transaction: TransactionData,
    invoice: Option<String>,
}

#[derive(Deserialize)]
struct TransactionData {
    id: String,
    amount: u64,
    amount_sats: String,
    exchange_rate: u64,
    status: String,
    payment_url: Option<String>,
}

#[derive(Deserialize)]
struct GetTransactionResponse {
    success: bool,
    transaction: Option<GetTransactionData>,
}

#[derive(Deserialize)]
struct GetTransactionData {
    status: String,
}

pub struct FlashApiGateway {
    client: Client,
    base_url: String,
    user_id: String,
    lightning_address: String,
}

impl FlashApiGateway {
    pub fn new(base_url: String, user_id: String, lightning_address: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            user_id,
            lightning_address,
        }
    }

    fn parse_sats(amount_sats: &str) -> u64 {
        amount_sats
            .split_whitespace()
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0)
    }
}

#[async_trait]
impl FlashGateway for FlashApiGateway {
    async fn create_sell_transaction(
        &self,
        amount_xof: &XofAmount,
        momo_number: &MomoNumber,
    ) -> Result<FlashTransaction, DomainError> {
        let url = format!("{}/transactions/create", self.base_url);

        let body = CreateTransactionBody {
            amount: amount_xof.amount(),
            receiver_address: self.lightning_address.clone(),
            transaction_type: "SELL_BITCOIN".to_string(),
            number: momo_number.value().to_string(),
        };

        let response = self.client
            .post(&url)
            .header("X-Staging-User-Id", &self.user_id)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        let data: CreateTransactionResponse = response
            .json()
            .await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        if !data.success {
            return Err(DomainError::ExternalService(
                "Flash API returned success: false".to_string()
            ));
        }

        let sats_value = Self::parse_sats(&data.transaction.amount_sats);

        let mut transaction = FlashTransaction::new(
            TransactionType::SellBitcoin,
            amount_xof.clone(),
            Satoshis::new(sats_value.max(1)).unwrap(),
            data.transaction.exchange_rate,
            momo_number.clone(),
        );

        transaction.set_flash_transaction_id(data.transaction.id.clone());

        // Stocke l'invoice Lightning
        if let Some(invoice) = data.invoice {
            transaction.set_invoice(invoice);
        }

        Ok(transaction)
    }

    async fn create_buy_transaction(
        &self,
        amount_xof: &XofAmount,
        momo_number: &MomoNumber,
        lightning_address: &str,
    ) -> Result<(FlashTransaction, Option<String>), DomainError> {
        let url = format!("{}/transactions/create", self.base_url);

        let body = serde_json::json!({
            "amount": amount_xof.amount(),
            "receiver_address": lightning_address,
            "type": "BUY_BITCOIN",
            "number": momo_number.value()
        });

        let response = self.client
            .post(&url)
            .header("X-Staging-User-Id", &self.user_id)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        let text = response.text().await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        tracing::info!("Flash BUY response: {}", text);

        let data: CreateTransactionResponse = serde_json::from_str(&text)
            .map_err(|e| DomainError::ExternalService(
                format!("Parse error: {} - Body: {}", e, text)
            ))?;

        if !data.success {
            return Err(DomainError::ExternalService(
                "Flash API returned success: false".to_string()
            ));
        }

        let sats_value = Self::parse_sats(&data.transaction.amount_sats);
        let payment_url = data.transaction.payment_url.clone();

        let mut transaction = FlashTransaction::new(
            TransactionType::BuyBitcoin,
            amount_xof.clone(),
            Satoshis::new(sats_value.max(1)).unwrap(),
            data.transaction.exchange_rate,
            momo_number.clone(),
        );
        transaction.set_flash_transaction_id(data.transaction.id.clone());

        Ok((transaction, payment_url))
    }

    async fn get_transaction_status(
        &self,
        transaction_id: &str,
    ) -> Result<String, DomainError> {
        let url = format!("{}/transactions/{}", self.base_url, transaction_id);

        let response = self.client
            .get(&url)
            .header("X-Staging-User-Id", &self.user_id)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        let data: GetTransactionResponse = response
            .json()
            .await
            .map_err(|e| DomainError::ExternalService(e.to_string()))?;

        let transaction = data.transaction
            .ok_or_else(|| DomainError::NotFound("Transaction not found on Flash".to_string()))?;

        Ok(transaction.status)
    }
}
