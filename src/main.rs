use dotenv::dotenv;
use reqwest::Client;
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

fn get_total_usage_cost(usage_records: &Vec<UsageRecord>) -> f64 {
    let mut usage_total = 0.0;
    let au_iter = usage_records.iter();
    for val in au_iter {
        let price: f64 = val.price.parse().unwrap();
        usage_total = usage_total + price;
    }

    return usage_total;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let account_sid = env::var("TWILIO_ACCOUNT_SID").unwrap();
    let auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();
    let request_url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{twilio_account_sid}/Usage/Records/LastMonth.json?PageSize={page_size}",
        twilio_account_sid = account_sid,
        page_size = 20
    );

    let client = Client::new();
    let response = client
        .get(&request_url)
        .basic_auth(account_sid, Some(auth_token))
        .send()
        .await?;

    let body = response
        .text()
        .await?;

    let account_usage: AccountUsage = serde_json::from_str(&body).unwrap();
    let usage_total = get_total_usage_cost(&account_usage.usage_records);

    println!("Twilio Account Usage");

    let data_iter = account_usage.usage_records.iter();
    for record in data_iter {
        println!("{},{},{},{}", record.start_date, record.end_date, record.category, record.price);
    }

    println!("Total records: {}", account_usage.page_size);
    println!("Total cost: {}", usage_total);

    Ok(())
}
