use crate::utils::spinner::create_spinner;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::env;


pub async fn execute(app_name: &str) -> Result<()> {

    let client = Client::new();
    dotenv::dotenv().ok();

    let nephelios_port: u16 = env::var("NEPHELIOS_PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .unwrap_or(3030);
    let nephelios_url =
        env::var("NEPHELIOS_URL").unwrap_or_else(|_| "http://localhost".to_string());
    let spinner = create_spinner(&format!("Removing {} application...", app_name));

    let payload = json!({
        "app_name": app_name,
    });

    let response = client
        .post(format!("{}:{}/remove", nephelios_url, nephelios_port))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    // Stop the spinner
    spinner.finish_and_clear();

    if response.status().is_success() {
        println!("✅ Removed app successfully: {:?}", app_name);
    } else {
        println!(
            "❌ Failed to remove app. Status: {}",
            response.status()
        );
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
    }

    Ok(())
}
