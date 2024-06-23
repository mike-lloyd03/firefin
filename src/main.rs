use anyhow::Result;
use chrono::{TimeZone, Utc};
use config::Config;
use types::Account;

mod config;
mod types;

fn main() -> Result<()> {
    let config = Config::load("config.toml")?;

    let start = Utc.with_ymd_and_hms(2024, 6, 16, 0, 0, 0).latest().unwrap();

    let accounts = Account::list(config, Some(start), None, None)?;

    for account in accounts {
        println!("{}", account.name);
        for transaction in account.transactions.unwrap_or_default() {
            println!(
                "{} {} on {}",
                transaction.description, transaction.amount, transaction.posted
            )
        }
    }

    Ok(())
}
