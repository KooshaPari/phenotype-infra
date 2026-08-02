//! Cloudflare DNS upsert — creates or updates an A record.

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use tracing::info;

use oci_helpers::expand_home;

#[derive(Debug, Serialize)]
struct ARecord<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
    name: &'a str,
    content: &'a str,
    ttl: u32,
    proxied: bool,
}

#[derive(Debug, Deserialize)]
struct ListResp {
    result: Vec<ExistingRecord>,
    success: bool,
}

#[derive(Debug, Deserialize)]
struct ExistingRecord {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ApiResp {
    success: bool,
    errors: serde_json::Value,
}

pub async fn upsert_a_record(zone_id: &str, token_file: &str, name: &str, ip: &str) -> Result<()> {
    upsert_a_record_at(
        "https://api.cloudflare.com/client/v4",
        zone_id,
        token_file,
        name,
        ip,
    )
    .await
}

async fn upsert_a_record_at(
    api_base: &str,
    zone_id: &str,
    token_file: &str,
    name: &str,
    ip: &str,
) -> Result<()> {
    let token = tokio::fs::read_to_string(expand_home(token_file))
        .await
        .with_context(|| format!("read CF token from {token_file}"))?;
    let token = token.trim();

    let client = reqwest::Client::new();
    // Find existing.
    let list_url = format!("{api_base}/zones/{zone_id}/dns_records?type=A&name={name}");
    let existing: ListResp = client
        .get(&list_url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    if !existing.success {
        return Err(anyhow!("cf list dns_records failed"));
    }

    let body = ARecord {
        kind: "A",
        name,
        content: ip,
        ttl: 60,
        proxied: false,
    };

    let resp: ApiResp = if let Some(rec) = existing.result.first() {
        let url = format!("{api_base}/zones/{zone_id}/dns_records/{}", rec.id);
        client
            .put(&url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?
            .json()
            .await?
    } else {
        let url = format!("{api_base}/zones/{zone_id}/dns_records");
        client
            .post(&url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?
            .json()
            .await?
    };
    if !resp.success {
        return Err(anyhow!("cf upsert failed: {}", resp.errors));
    }
    info!(name, ip, "cloudflare A record upserted");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn temp_file(name: &str, contents: &[u8]) -> String {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("oci-cf-{name}-{nonce}"));
        std::fs::write(&path, contents).unwrap();
        path.to_string_lossy().into_owned()
    }

    async fn serve_json(responses: Vec<&'static str>) -> String {
        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move {
            for body in responses {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = [0u8; 4096];
                let _ = stream.read(&mut request).await.unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        base
    }

    #[tokio::test]
    async fn upsert_updates_an_existing_record() {
        let base = serve_json(vec![
            r#"{"success":true,"result":[{"id":"record-1"}]}"#,
            r#"{"success":true,"errors":[]}"#,
        ])
        .await;
        let token = temp_file("token", b"token\n");
        upsert_a_record_at(&base, "zone", &token, "node.example", "198.51.100.10")
            .await
            .unwrap();
        let _ = std::fs::remove_file(token);
    }

    #[tokio::test]
    async fn upsert_creates_when_no_record_exists() {
        let base = serve_json(vec![
            r#"{"success":true,"result":[]}"#,
            r#"{"success":true,"errors":[]}"#,
        ])
        .await;
        let token = temp_file("token", b"token");
        upsert_a_record_at(&base, "zone", &token, "node.example", "198.51.100.11")
            .await
            .unwrap();
        let _ = std::fs::remove_file(token);
    }

    #[tokio::test]
    async fn upsert_reports_cloudflare_failures() {
        let base = serve_json(vec![r#"{"success":false,"result":[]}"#]).await;
        let token = temp_file("token", b"token");
        let err = upsert_a_record_at(&base, "zone", &token, "node.example", "198.51.100.12")
            .await
            .unwrap_err();
        assert!(err.to_string().contains("list dns_records"));
        let _ = std::fs::remove_file(token);
    }
}
