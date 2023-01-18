use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::ACCEPT;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct SubresourceUris {
    all_time: String,
    today: String,
    yesterday: String,
    this_month: String,
    last_month: String,
    daily: String,
    monthly: String,
    yearly: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsageRecord {
    category: String,
    description: String,
    account_sid: String,
    start_date: String,
    end_date: String,
    as_of: String,
    count: String,
    count_unit: String,
    usage: String,
    usage_unit: String,
    price: String,
    price_unit: String,
    api_version: String,
    uri: String,
    subresource_uris: SubresourceUris,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountUsage {
    first_page_uri: String,
    last_page_uri: Option<String>,
    next_page_uri: Option<String>,
    previous_page_uri: Option<String>,
    num_pages: Option<u8>,
    page: u8,
    page_size: u8,
    start: u8,
    end: u8,
    total: Option<u8>,
    uri: String,
    usage_records: Vec<UsageRecord>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let account_sid = env::var("TWILIO_ACCOUNT_SID").unwrap();
    let auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();
    let password: Option<String> = Some(auth_token);
    let base_uri = "https://api.twilio.com/2010-04-01";

    let request_url = format!(
        "{base_uri}/Accounts/{twilio_account_sid}/Usage/Records/LastMonth.json?PageSize={page_size}",
        base_uri = base_uri,
        twilio_account_sid = account_sid,
        page_size = 20
    );

    let client = Client::new();
    let response = client
        .get(&request_url)
        .header(ACCEPT, "application/json")
        .basic_auth(account_sid, password)
        .send()
        .await?;

    let body = response
        .text()
        .await?;

    let account_usage: AccountUsage = serde_json::from_str(&body).unwrap();

    println!("Page size is {}", account_usage.page_size);

    Ok(())
}