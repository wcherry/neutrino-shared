use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest<'a> {
    name: &'a str,
    endpoint: &'a str,
    version: &'a str,
    health_check_url: String,
}

pub async fn register_with_drive(
    drive_base_url: &str,
    service_name: &str,
    service_endpoint: &str,
    version: &str,
) -> Result<(), String> {
    let url = format!("{}/api/v1/internal/services/register", drive_base_url);
    let body = RegisterRequest {
        name: service_name,
        endpoint: service_endpoint,
        version,
        health_check_url: format!("{}/health", service_endpoint),
    };
    let resp = Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to reach drive for service registration: {}", e))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Drive service registration returned {}",
            resp.status()
        ))
    }
}
