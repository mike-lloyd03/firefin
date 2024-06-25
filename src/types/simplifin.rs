use anyhow::Result;
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use reqwest::{blocking::get, Url};
use serde::Deserialize;

use crate::{
    config::Config,
    types::utils::{as_f64, as_f64_opt},
};

const BASE_URL: &str = "https://beta-bridge.simplefin.org/simplefin/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub currency: String,
    #[serde(deserialize_with = "as_f64")]
    pub balance: f64,
    #[serde(deserialize_with = "as_f64_opt")]
    pub available_balance: Option<f64>,
    #[serde(with = "ts_seconds")]
    pub balance_date: DateTime<Utc>,
    pub transactions: Option<Vec<Transaction>>,
    pub org: Org,
}

impl Account {
    pub fn list(
        config: &Config,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        pending: Option<bool>,
    ) -> Result<Vec<Self>> {
        #[derive(Deserialize)]
        struct AccountsResp {
            accounts: Vec<Account>,
        }
        let req_url = Url::parse(BASE_URL)?;
        let mut req_url = req_url.join("accounts")?;
        req_url
            .set_username(&config.username)
            .expect("Should set username");
        req_url
            .set_password(Some(&config.password))
            .expect("Should set password");

        if let Some(start) = start {
            req_url
                .query_pairs_mut()
                .append_pair("start-date", &start.timestamp().to_string())
                .finish();
        }

        if let Some(end) = end {
            req_url
                .query_pairs_mut()
                .append_pair("end-date", &end.timestamp().to_string())
                .finish();
        }

        if let Some(pending) = pending {
            req_url
                .query_pairs_mut()
                .append_pair("pending", &pending.to_string())
                .finish();
        }

        let resp: AccountsResp = get(req_url)?.json()?;

        Ok(resp.accounts)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Org {
    pub domain: Option<String>,
    pub sfin_url: String,
    pub name: Option<String>,
    pub url: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub id: String,
    #[serde(with = "ts_seconds")]
    pub posted: DateTime<Utc>,
    #[serde(deserialize_with = "as_f64")]
    pub amount: f64,
    pub description: String,
    #[serde(with = "ts_seconds_option")]
    pub transacted_at: Option<DateTime<Utc>>,
    pub pending: Option<bool>,
    pub memo: Option<String>,
}
