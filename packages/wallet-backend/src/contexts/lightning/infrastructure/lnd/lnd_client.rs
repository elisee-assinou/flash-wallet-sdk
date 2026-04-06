use tokio_stream::StreamExt;

pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

pub struct LndClient {
    client: tonic_lnd::Client,
}

impl LndClient {
    pub async fn connect(
        address: &str,
        tls_cert_path: &str,
        macaroon_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = tonic_lnd::connect(
            address.to_string(),
            tls_cert_path,
            macaroon_path,
        ).await?;

        Ok(Self { client })
    }

    pub async fn subscribe_invoices(
        &mut self,
        mut callback: impl FnMut(tonic_lnd::lnrpc::Invoice),
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lightning = self.client.lightning().clone();

        let response = lightning
            .subscribe_invoices(tonic_lnd::lnrpc::InvoiceSubscription {
                add_index: 0,
                settle_index: 0,
            })
            .await?;

        let mut stream = response.into_inner();

        while let Some(invoice) = stream.next().await {
            match invoice {
                Ok(inv) if inv.state == 1 => callback(inv),
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Stream error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn add_invoice(
        &mut self,
        amount_sats: i64,
        memo: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut lightning = self.client.lightning().clone();

        let response = lightning
            .add_invoice(tonic_lnd::lnrpc::Invoice {
                value: amount_sats,
                memo: memo.to_string(),
                ..Default::default()
            })
            .await?
            .into_inner();

        Ok(response.payment_request)
    }

    pub async fn pay_invoice(
        &mut self,
        payment_request: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lightning = self.client.lightning().clone();

        lightning
            .send_payment_sync(tonic_lnd::lnrpc::SendRequest {
                payment_request: payment_request.to_string(),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn list_settled_invoices_for_momo(
        &mut self,
        momo_number: &str,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut lightning = self.client.lightning().clone();
        let memo_prefix = format!("flash-wallet:{}", momo_number);

        let response = lightning
            .list_invoices(tonic_lnd::lnrpc::ListInvoiceRequest {
                pending_only: false,
                index_offset: 0,
                num_max_invoices: 10000,
                reversed: false,
            })
            .await?
            .into_inner();

        let total_sats: u64 = response.invoices
            .iter()
            .filter(|inv| inv.state == 1 && inv.memo.starts_with(&memo_prefix))
            .map(|inv| inv.value as u64)
            .sum();

        Ok(total_sats)
    }
}
