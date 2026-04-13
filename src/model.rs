use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InvoiceStatus {
    Processing = 0,
    Paid = 1,
    Unpaid = 2,
    Refund = 3,
    Unknown = 4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub subscription_id: i64,
    pub amount: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_refund: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {}
