use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub pay_system_id: String,
    pub plan_id: String,
    pub amount: f64,
    pub currency: String,
    pub cancel_at_period_end: bool,
    pub customer: String,
    pub status: String,
    pub user_id: String,
    pub charges_count: i32,
    pub total_revenue: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiz_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paywall_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_charge_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_charge_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_visit_id: Option<String>,

    /// Projected revenue over 32 days (e.g. in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_32d: Option<i32>,

    /// Projected revenue over 62 days (e.g. in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_62d: Option<i32>,

    /// Projected revenue over 184 days (e.g. in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_184d: Option<i32>,

    /// Projected revenue over 367 days (e.g. in cents)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_367d: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriptionResponse {
    pub success: i32,
    pub data: SubscriptionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionData {
    pub id: i32,
    pub user_id: String,
    pub user_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_system: i32,
    pub payment_system_label: String,
    pub real_payment: i32,
    pub pay_system_id: String,
    pub project_domain: String,
    pub quiz_id: String,
    pub quiz_name: String,
    pub paywall_id: String,
    pub paywall_name: String,
    pub price_id: String,
    pub amount: f64,
    pub amount_real: f64,
    pub currency: String,
    pub customer: String,
    pub status: SubscriptionStatus,
    pub charges_count: i32,
    pub total_revenue: i32,
    pub price: Price,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_visit: Option<UserVisit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_charge_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_charge_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_32d: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_62d: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_184d: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_revenue_367d: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_api_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserVisit {
    // structure unknown → keep flexible
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub id: i32,
    pub plan_id: i32,
    pub external_id: String,
    pub currency: String,
    pub amount: String,
    pub period: String,
    pub discount_duration_in_billing_periods: String,
    pub amount_real: f64,
    pub price_option_text: String,
    pub plan: Plan,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_real_with_discount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub project_id: i32,
    pub payment_system: String,
    pub external_id: String,
    pub name: String,
    pub description: String,
    pub livemode: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Incomplete,
    IncompleteExpired,
    Trialing,
    PastDue,
    Canceled,
    Unpaid,
    Paused,

    #[serde(other)]
    Unknown,
}
