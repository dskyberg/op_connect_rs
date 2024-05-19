use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::{Error, Result};
use crate::{Item, Vault};

#[derive(Debug)]
pub struct OpClient {
    api_key: Zeroizing<String>,
    path: Url,
    client: Client,
}

impl OpClient {
    pub fn new(path: &str, api_key: &str) -> Result<Self> {
        let client = Client::new();
        let api_key = Zeroizing::new(api_key.to_owned());
        let path = Url::try_from(path).map_err(|_e| Error::UrlParse)?;
        Ok(Self {
            api_key,
            path,
            client,
        })
    }

    fn join(&self, part: &str) -> Result<Url> {
        self.path.join(part).map_err(|_e| Error::UrlParse)
    }

    async fn get<T: for<'a> Deserialize<'a>>(&self, path: &str) -> Result<T> {
        let path = self.join(path)?;
        let response = self
            .client
            .get(path)
            .header("Accept", "application/json")
            .bearer_auth((*self.api_key).clone())
            .send()
            .await?
            .text()
            .await?;
        let result = serde_json::from_str::<T>(&response)?;
        Ok(result)
    }

    #[allow(dead_code)]
    async fn post<T: for<'a> Deserialize<'a>, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let path = self.join(path)?;

        let response = self
            .client
            .post(path)
            .json(body)
            .header("Accept", "application/json")
            .bearer_auth((*self.api_key).clone())
            .send()
            .await?
            .text()
            .await?;
        let result = serde_json::from_str::<T>(&response)?;
        Ok(result)
    }

    pub async fn get_vaults(&self) -> Result<Vec<Vault>> {
        self.get::<Vec<Vault>>("vaults").await
    }

    pub async fn get_vault_by_id(&self, vault_id: &str) -> Result<Vault> {
        self.get::<Vault>(&format!("vaults/{}", vault_id)).await
    }

    pub async fn get_vault_by_name(&self, name: &str) -> Result<Vault> {
        let vaults = self
            .get::<Vec<Vault>>(&format!("vaults?name=\"{}\"", name))
            .await?;
        if vaults.len() == 1 {
            Ok(vaults[0].clone())
        } else {
            // this is an error
            Err(Error::NotFound)
        }
    }

    pub async fn get_items(&self, vault: &Vault) -> Result<Vec<Item>> {
        let path = format!("vaults/{}/items", vault.id);
        self.get::<Vec<Item>>(&path).await
    }

    pub async fn get_item_detail(&self, vault: &Vault, item_id: &str) -> Result<Item> {
        let path = format!("vaults/{}/items/{}", vault.id, item_id);
        self.get::<Item>(&path).await
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn get_client() -> OpClient {
        let path = std::env::var("OP_PATH").unwrap_or("http://localhost:8080/v1/".to_owned());
        let api_key = std::env::var("OP_API_TOKEN").expect("OP_API_TOKEN not set");

        OpClient::new(&path, &api_key).expect("failed to create client")
    }

    #[tokio::test]
    async fn test_get_items() {
        let client = get_client();
        let vault = client
            .get_vault_by_name("dev")
            .await
            .expect("failed to get vault");
        let result = client.get_items(&vault).await;
        assert!(result.is_ok());
        println!("Result: {:#?}", &result.unwrap());
    }

    #[tokio::test]
    async fn test_get_item_detail() {
        let client = get_client();
        let vault = client
            .get_vault_by_name("dev")
            .await
            .expect("failed to get vault");
        let items = client.get_items(&vault).await.expect("failed to get items");
        let item = items.last().expect("no items");
        let item_detail = client
            .get_item_detail(&vault, &item.id)
            .await
            .expect("failed to get items");
        //println!("Item Details: {:#?}", &item);
        println!(
            "Username: {}",
            *item_detail.get_field_by_id("username").unwrap()
        );
        println!(
            "Credential: {:?}",
            *item_detail.get_field_by_id("credential").unwrap()
        );
        println!(
            "Okta App: {:?}",
            *item_detail.get_field_by_label("app").unwrap()
        );
    }

    #[tokio::test]
    async fn test_vaults() {
        println!(
            "Vaults: {:#?}",
            get_client().get_vaults().await.expect("oops")
        )
    }

    #[tokio::test]
    async fn test_vault_by_id() {
        let client = get_client();
        let vault = client.get_vault_by_id("2me2svdjkmsh5xmqodspni4h6a").await;
        println!("Result: {:#?}", vault);
        //println!("Vault: {:#?}", &vault);
    }

    #[tokio::test]
    async fn test_vault_by_name() {
        let client = get_client();
        let vault = client.get_vault_by_name("dev").await;
        println!("Result: {:#?}", vault);
        //println!("Vault: {:#?}", &vault);
    }
}
