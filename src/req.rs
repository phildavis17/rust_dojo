use reqwest::Error;
use std::collections::HashMap;

pub async fn get_check() -> Result<(), Error> {
    let url = "https://api.weather.gov/alerts/types";
    let client = reqwest::Client::builder()
        .user_agent("TESTING APIs")
        .build()?;
    let body = client.get(url)
        .send()
        .await?
        .text()
        .await?;
    println!("{body:?}");
    Ok(())
}



pub async fn get_check_2() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}