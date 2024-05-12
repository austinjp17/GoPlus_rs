use reqwest::{self, Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod api_structs;
use api_structs::*;

const BASE_URL: &str = "https://api.gopluslabs.io/api/v1";
pub struct Session {
    inner: Client,
    access_token: Option<String>,
}

// pub fn handle_res(res: Response) -> Result<Value, Error> {

// }

pub enum Error {
    RequestError(reqwest::Error),
    ParseError,
    StatusError,
}


impl Session {
    pub fn new() -> Self {
        Self {
            inner: Client::new(),
            access_token: None,
        }
    }

    pub async fn supported_chains(&self, access_token: Option<&str>) -> Result<SupportedChainsResponse, anyhow::Error> {
        let url = format!("{BASE_URL}/supported_chains");
        let token = match access_token {
            Some(t) => t,
            None => "None",
        };

        Ok(self
            .inner
            .get(url)
            .header("access_token", token)
            .send()
            .await?
            .json::<SupportedChainsResponse>()
            .await?)
    }

    pub async fn token_security(&self, chain_id: &str, addr: &str, access_token: Option<&str>) -> Result<TokenResponse, anyhow::Error> {
        let url = format!(
            "{}/token_security/{}", BASE_URL, chain_id
        );

        let token = match access_token {
            Some(t) => t,
            None => "None",
        };

        Ok(self.inner.get(url)
            .header("access_token", token)
            .query(&[("contract_addresses", addr)])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?)
    }

    pub async fn address_security(&self, addr: &str, chain_id: Option<&str>, access_token: Option<&str>) -> Result<AccountResponse, anyhow::Error> {
        let url = format!("{}/address_security/{}", BASE_URL, addr);

        let access_token = match access_token {
            Some(t) => t,
            None => "None",
        };

        let chain_id = match chain_id {
            Some(id) => id,
            None => "None",
        };

        Ok(self.inner.get(url)
            .header("access_token", access_token)
            .query(&[("chain_id", chain_id)])
            .send()
            .await?
            .json::<AccountResponse>()
            .await?)
    }

}
