use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::utils::{to_string, to_string_opt};

use super::simplifin;

#[derive(Debug, Default, Serialize)]
enum TransactionType {
    #[default]
    Withdrawal,
    Deposit,
    Transfer,
    Reconciliation,
    OpeningBalance,
}

#[derive(Debug, Default, Serialize)]
pub struct Transaction {
    r#type: TransactionType,
    date: DateTime<Utc>,
    #[serde(serialize_with = "to_string")]
    amount: f64,
    description: String,
    order: Option<i64>,
    currency_id: Option<String>,
    currency_code: Option<String>,
    #[serde(serialize_with = "to_string_opt")]
    foreign_amount: Option<f64>,
    #[serde(serialize_with = "to_string_opt")]
    foreign_currency_id: Option<i64>,
    foreign_currency_code: Option<String>,
    #[serde(serialize_with = "to_string_opt")]
    budget_id: Option<i64>,
    #[serde(serialize_with = "to_string_opt")]
    category_id: Option<i64>,
    category_name: Option<String>,
    #[serde(serialize_with = "to_string_opt")]
    source_id: Option<i64>,
    source_name: Option<String>,
    #[serde(serialize_with = "to_string_opt")]
    destination_id: Option<i64>,
    destination_name: Option<String>,
    reconciled: Option<bool>,
    piggy_bank_id: Option<i64>,
    piggy_bank_name: Option<String>,
    #[serde(serialize_with = "to_string_opt")]
    bill_id: Option<i64>,
    bill_name: Option<String>,
    // tags: null,
    notes: Option<String>,
    internal_reference: Option<String>,
    external_id: Option<String>,
    external_url: Option<String>,
    bunq_payment_id: Option<String>,
    sepa_cc: Option<String>,
    sepa_ct_op: Option<String>,
    sepa_ct_id: Option<String>,
    sepa_db: Option<String>,
    sepa_country: Option<String>,
    sepa_ep: Option<String>,
    sepa_ci: Option<String>,
    sepa_batch_id: Option<String>,
    interest_date: Option<DateTime<Utc>>,
    book_date: Option<DateTime<Utc>>,
    process_date: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
    payment_date: Option<DateTime<Utc>>,
    invoice_date: Option<DateTime<Utc>>,
}

impl From<simplifin::Transaction> for Transaction {
    fn from(value: simplifin::Transaction) -> Self {
        Self {
            external_id: Some(value.id),
            date: value.posted,
            amount: value.amount,
            description: value.description,
            notes: value.memo,
            r#type: if value.amount > 0.0 {
                TransactionType::Deposit
            } else {
                TransactionType::Withdrawal
            },
            ..Default::default()
        }
    }
}
