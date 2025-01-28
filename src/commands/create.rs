use crate::utils::spinner::create_spinner;
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

/// Executes the create command to deploy a new application.
///
/// # Arguments
///
/// * `app_name` - The name of the application to deploy
/// * `app_type` - The type of application (e.g., nodejs, python, rust)
/// * `github_url` - The GitHub repository URL containing the application code
///
/// # Returns
///
/// * `Ok(())` if the deployment was successful
/// * `Err(anyhow::Error)` if there was an error during deployment
///
/// # Examples
///
/// ```
/// let result = execute("my-app", "nodejs", "https://github.com/user/repo").await;
/// match result {
///     Ok(_) => println!("Deployment successful"),
///     Err(e) => println!("Deployment failed: {}", e),
/// }
/// ```
pub async fn execute(app_name: &str, app_type: &str, github_url: &str) -> Result<()> {
    let client = Client::new();

    let spinner = create_spinner(&format!("Deploying {} application...", app_name));

    let payload = json!({
        "app_name": app_name,
        "app_type": app_type,
        "github_url": github_url,
    });

    let response = client
        .post("http://127.0.0.1:3030/create")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    // Stop the spinner
    spinner.finish_and_clear();

    if response.status().is_success() {
        let response_body: Value = response.json().await?;
        println!("✅ Deployment created successfully: {:?}", response_body);
    } else {
        println!(
            "❌ Failed to create deployment. Status: {}",
            response.status()
        );
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
    }

    Ok(())
}
