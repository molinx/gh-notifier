use serde_json::Value;
use tauri_plugin_http::reqwest;

pub async fn gh_request(client: &reqwest::Client) -> Result<usize, reqwest::Error> {
    let token = env!("TOKEN");
    let response = client
        .get("https://api.github.com/notifications")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "GH-Noti")
        .send()
        .await?;

    let notifications: Vec<Value> = response.json().await?;

    Ok(notifications.len())
}
