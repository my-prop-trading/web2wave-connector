use std::time::Duration;
use web2wave_connector::{api::Web2WaveApi, model::CreateInvoiceRequest};

#[tokio::main]
async fn main() {
    let base_url = "https://test.com";
    let api_key = std::env::var("API_KEY").unwrap();
    let api = Web2WaveApi::new(base_url, api_key, Duration::from_secs(15));

    let result = api
        .create_invoice(&CreateInvoiceRequest {
            subscription_id: 1,
            amount: 10.0,
            invoice_id: None,
            status: None,
            date: None,
            amount_usd: None,
            is_refund: None,
        })
        .await;
    println!("{result:?}");

    println!("RUN basic example: OK");
}
