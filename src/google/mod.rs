use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use googapis::clients::auth::TokenProvider;
use googapis::clients::firestore::v1;
use googapis::google::firestore::v1::GetDocumentRequest;
use reqwest::Client;
use serde::Deserialize;
use urlencoding::encode;

use crate::jwt;

pub struct Firestore {}

impl Firestore {
    pub async fn get_document(collection: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let claims = jwt::claims(token).await?;
        let provider = Provider { token: token.to_string() };
        let name = format!("projects/{}/databases/(default)/documents/{}/{}", claims.aud, collection, claims.user_id);
        let mut client = v1::client::get_client::<Provider, anyhow::Error>(&claims.aud, provider, Some(Duration::from_secs(30))).await?;
        let response = client.get_document(GetDocumentRequest {
            name,
            mask: None,
            consistency_selector: None,
        }).await.unwrap().into_inner();

        // firestore_serde::from_document::<HashMap<String, Value>>(response);
        println!("{:#?}", response);
        Ok(())
    }
}

struct Provider {
    token: String,
}

#[async_trait]
impl TokenProvider for Provider {
    async fn token(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageList {
    items: Vec<HashMap<String, String>>,
    next_page_token: Option<String>,
}

pub struct Storage {}

impl Storage {
    pub async fn ls(bucket: &str, prefix: &str, jwt: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut prefix = prefix.to_string();
        if !prefix.ends_with('/') {
            prefix = format!("{}/", prefix);
        }
        let _url = "https://firebasestorage.googleapis.com/v0/b/artifactosaurus/o/rpms%2Fcerberus-2.0.0-1.aarch64.rpm?alt=media";
        let url = format!("https://firebasestorage.googleapis.com/v0/b/{}/o/?prefix={}", bucket, encode(&prefix));

        let mut next_page = None;

        loop {
            let mut request = Client::new().get(&url).header("authorization", format!("Bearer {}", jwt));
            if next_page.is_some() {
                request = request.query(&[("pageToken", next_page.unwrap())]);
            }
            let list = request.send().await?.json::<StorageList>().await?;
            list.items.into_iter().for_each(|item| item.get("name").iter().for_each(|v| println!("{}", v)));

            next_page = list.next_page_token;

            if next_page.is_none() {
                break;
            }
        }

        Ok(())
    }
}