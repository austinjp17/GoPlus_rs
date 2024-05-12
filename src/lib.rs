use reqwest::Client;


mod api_structs;
use api_structs::*;
use serde_json::{json, Value};

const BASE_URL: &str = "https://api.gopluslabs.io/api/v1";


pub struct Session {
    inner: Client,
    #[allow(dead_code)]
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

    /// Obtains an access token using SHA-1 signature method.
    ///
    /// # Sign Method
    /// Concatenate `app_key`, `time`, and `app_secret` in turn, and apply SHA-1 hashing.
    ///
    /// # Example
    /// ```
    /// let app_key = "mBOMg20QW11BbtyH4Zh0";
    /// let time = 1647847498;
    /// let app_secret = "V6aRfxlPJwN3ViJSIFSCdxPvneajuJsh";
    /// let sign = "sha1(mBOMg20QW11BbtyH4Zh01647847498V6aRfxlPJwN3ViJSIFSCdxPvneajuJsh)"; // This results in `7293d385b9225b3c3f232b76ba97255d0e21063e`
    /// ```
    ///
    /// # Parameters
    /// - `app_key`: Application key provided by the service.
    /// - `signature`: Computed SHA-1 hash as a string.
    /// - `time`: Current time as a UNIX timestamp.
    ///
    /// # Example Usage
    /// ```
    /// let mut instance = Session::new(None);
    /// instance.get_access_token("mBOMg20QW11BbtyH4Zh0", "7293d385b9225b3c3f232b76ba97255d0e21063e", 1647847498);
    /// ```
    pub async fn get_access_token(&mut self, app_key: &str, signature: &str, time: u64) -> Result<(), anyhow::Error> {
        let url = format!("{}/token", BASE_URL);
        // How to do body params?

        let params = json!({
            "app_key": app_key,
            "sign": signature,
            "time": time,
        });

        let access_code_res = self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .json(&params)
            .send()
            .await?
            .json::<AccessCodeResponse>()
            .await?;

        // access_code_res.result.unwrap().expires_in;
        if access_code_res.code == 1 {
            tracing::info!("New access token expires in {} minutes", (access_code_res.result.as_ref().unwrap().expires_in)/60);

            self.access_token = Some(access_code_res.result.unwrap().access_token);
        } else {
            tracing::error!("Error getting access token\nCode: {}", access_code_res.code)
            // ERROR HANDLING
        };
        
        Ok(())

    }
    
    pub async fn supported_chains(&self) -> Result<SupportedChainsResponse, anyhow::Error> {
        let url = format!("{BASE_URL}/supported_chains");

        Ok(self
            .inner
            .get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .send()
            .await?
            .json::<SupportedChainsResponse>()
            .await?)
    }

    pub async fn token_risk(&self, chain_id: &str, addr: &str) -> Result<TokenResponse, anyhow::Error> {
        let url = format!(
            "{}/token_security/{}", BASE_URL, chain_id
        );

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("contract_addresses", addr)])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?)
    }

    pub async fn address_risk(&self, addr: &str, chain_id: Option<&str>) -> Result<AccountResponse, anyhow::Error> {
        let url = format!("{}/address_security/{}", BASE_URL, addr);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("chain_id", chain_id.unwrap_or("None"))])
            .send()
            .await?
            .json::<AccountResponse>()
            .await?)
    }

    pub async fn approval_security_v1(&self, chain_id: &str, contract_addr: &str) -> Result<ASresponse, anyhow::Error> {
        let url = format!("{}/approval_security/{}", BASE_URL, chain_id);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("contract_addresses", contract_addr)])
            .send()
            .await?
            .json::<ASresponse>()
            .await?)
    }

    // TODO
    pub async fn approval_security_v2(&self) {
        todo!();
    }

    pub async fn abi_decode(&self, 
        chain_id: &str, 
        data: &str,
        contract_addr: Option<&str>,
        signer: Option<&str>,
        txn_type: Option<&str>
    ) -> Result<AbiDecodeResponse, anyhow::Error> {
        
        let url = format!("{}/abi/input_decode", BASE_URL);

        let params = json!({
            "chain_id": chain_id,
            "data": data,
            "contract_address": contract_addr,
            "signer": signer,
            "transaction_type": txn_type
        });

        Ok(self.inner.post(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .json(&params)
            .send()
            .await?
            .json::<AbiDecodeResponse>()
            .await?)
    }


    pub async fn nft_risk(&self, chain_id: &str, contract_addr: &str, token_id: Option<&str>) -> Result<NftRiskResponse, anyhow::Error> {
        let url = format!("{}/nft_security/{}",BASE_URL, chain_id);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("contract_addresses", contract_addr), ("token_id", token_id.unwrap_or("None"))])
            .send()
            .await?
            .json::<NftRiskResponse>()
            .await?)
    }

    // TODO: No successfully found url
    pub async fn dapp_risk_by_url(&self, dapp_url: &str) -> Result<Value, anyhow::Error> {
        todo!("Fails on all tried urls idk");
        let url = format!("{}/dapp_security", BASE_URL);
        
        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("url", dapp_url)])
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    pub async fn phishing_site_risk(&self, site_url: &str) -> Result<PhishingSiteResponse, anyhow::Error> {
        let url = format!("{}/phishing_site", BASE_URL);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("url", site_url)])
            .send()
            .await?
            .json::<PhishingSiteResponse>()
            .await?)
    }
    
    pub async fn rug_pull_risk(&self, chain_id: &str, contract_addr: &str) -> Result<RugPullRiskResponse, anyhow::Error> {
        let url = format!("{}/rugpull_detecting/{}", BASE_URL, chain_id);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("contract_addresses", contract_addr)])
            .send()
            .await?
            .json::<RugPullRiskResponse>()
            .await?)
    }
}



pub fn interpret_status_code(code: u32) -> &'static str {
    match code {
        1 => "Complete data prepared",
        2 => "Partial data obtained. The complete data can be requested again in about 15 seconds.",
        2004 => "Contract address format error!",
        2018 => "ChainID not supported",
        2020 => "Non-contract address",
        2021 => "No info for this contract",
        2022 => "Non-supported chainId",
        2026 => "dApp not found",
        2027 => "ABI not found",
        2028 => "The ABI not support parsing",
        4010 => "App_key not exist",
        4011 => "Signature expiration (the same request parameters cannot be requested more than once)",
        4012 => "Wrong Signature",
        4023 => "Access token not found",
        4029 => "Request limit reached",
        5000 => "System error",
        5006 => "Param error!",
        _ => "Unknown status code",
    }
}