use std::error::Error;

use reqwest;

pub async fn get_current_ip(base_url: &str) -> Result<String, Box<dyn Error>> {
    match reqwest::get(format!("{base_url}/ip")).await?.text().await {
        Ok(r) => Ok(r),
        Err(_) => {
            return Err("Couldn't find current IP".into());
        }
    }
}

pub async fn update_host(
    base_url: &str,
    username: &str,
    password: &str,
    host: &str,
    ip: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    match client
        .get(format!("{base_url}/update/?host={host}&ip={ip}"))
        .basic_auth(username, Some(password))
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(r) => {
                if !r.contains("ok") {
                    return Err(format!("Something went wrong updating the host: {}", r).into());
                }
                return Ok(r);
            }
            Err(e) => {
                return Err(format!("Couldn't parse response: {}", e).into());
            }
        },
        Err(e) => {
            return Err(format!("Something went terrible wrong! Error: {}", e).into());
        }
    }
}
