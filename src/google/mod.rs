use std::time::Duration;

use async_trait::async_trait;
use googapis::clients::auth::TokenProvider;
use googapis::clients::firestore::v1;
use googapis::google::firestore::v1::GetDocumentRequest;

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