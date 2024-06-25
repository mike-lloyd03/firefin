use anyhow::Result;
use chrono::{TimeZone, Utc};
use config::Config;
use reqwest::Url;
use serde::Serialize;
use types::{firefly::Transaction, simplifin::Account};

mod config;
mod types;

#[derive(Serialize)]
struct TransactionPost {
    transactions: Vec<Transaction>,
}

fn main() -> Result<()> {
    let config = Config::load("config.toml")?;

    let start = Utc.with_ymd_and_hms(2024, 6, 16, 0, 0, 0).latest().unwrap();

    let accounts = Account::list(&config, Some(start), None, Some(true))?;

    let client = reqwest::Client::new();
    let firfly_url = Url::parse(&config.firefly_url)?;

    // let transactions_to_post = vec![];

    for account in accounts {
        println!(
            "Name: {}, Balance: {}, Available balance: {}",
            account.name,
            account.balance,
            account.available_balance.unwrap_or_default()
        );
        for transaction in account.transactions.unwrap_or_default() {
            println!(
                "  - {} {} on {}",
                transaction.description, transaction.amount, transaction.posted
            );
        }
    }

    // client.post(firfly_url.join("/v1/transactions"));

    Ok(())
}
