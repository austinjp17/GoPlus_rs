// TODO: Error handling
use std::{sync::Arc, time::UNIX_EPOCH};
use reqwest::Client;
mod api_structs;
use api_structs::*;
use serde_json::{json, Value};

const BASE_URL: &str = "https://api.gopluslabs.io/api/v1";

pub enum GpError {
    RequestError(reqwest::Error),
    ParseError,
    StatusError,
}

#[derive(Default)]
pub struct Session {
    inner: Client,
    access_token: Option<String>,
}

// TODO
pub enum V2ApprovalERC {
    ERC20,
    ERC721,
    ERC1155
}

impl Session {
    pub fn new() -> Self {
        // If app_key env var set
        let app_key = std::env::var("GP_PUBLIC");
        let secret_key = std::env::var("GP_SECRET");

        if app_key.is_err() || secret_key.is_err(){
            // No access token
            tracing::warn!("Set enviornment variables to get access code");
            tracing::warn!("  `export GP_PUBLIC = $APP_PUBLIC_KEY$`");
            tracing::warn!("  `export GP_PUBLIC = $APP_PRIVATE_KEY$`");
            Self {
                inner: Client::new(),
                access_token: None,
            }
        } 
        else {
            // UNCERTAIN IF WORKS, CAN'T TEST W/OUT KEYS
            use sha1::{Sha1, Digest};
            let mut hasher = Sha1::new();
            let time: u64 = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let hash_str = format!("{}{}{}", app_key.unwrap(), time, secret_key.unwrap());
            hasher.update(hash_str);
            let f = hasher.finalize();
            let str_hash = format!("{:x}", f);
            
            Self {
                inner: Client::new(),
                access_token: Some(str_hash),
            }
        }

        
    }

    /// Retrieves a list of supported blockchain chains from the API.
    ///
    /// 
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.supported_chains().await?;
    /// let chains: Vec<Chain> = response.result;
    /// ```
    /// Tablular form of return data available [here](https://docs.gopluslabs.io/reference/response-details-9)
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

    /// Fetches token risk data based on the blockchain chain ID and address.
    ///
    /// # Parameters
    /// - `chain_id`: The blockchain chain ID.
    /// - `addr`: The address to check.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.token_risk("56", "0xEa51801b8F5B88543DdaD3D1727400c15b209D8f").await?;
    /// let risk_data: Hashmap<String, TokenData> = response.result;
    /// ```
    /// Response fields in depth [here](https://docs.gopluslabs.io/reference/response-details)
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

    /// Retrieves risk information about an address, optionally filtered by chain ID.
    ///
    /// If only the address is provided without specifying the chain ID, the `contract_address` 
    /// field in the response may be omitted. This occurs because the same address can represent 
    /// a contract on one blockchain but not on another. Determination of `contract_address` involves
    /// querying a third-party blockchain browser interface, which may delay the response. 
    /// The `contract_address` field may initially be empty due to this delay. A subsequent request 
    /// after about 5 seconds typically returns complete data, including the `contract_address`.
    ///
    /// # Parameters
    /// - `addr`: The address to analyze.
    /// - `chain_id`: Optional blockchain chain ID for filtering.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.address_risk("0xEa51801b8F5B88543DdaD3D1727400c15b209D8f", Some("56")).await;
    /// let risk_data: AccountRisk = response.result;
    /// ```
    /// Response fields in depth [here](https://docs.gopluslabs.io/reference/response-details-1)
    pub async fn address_risk(&self, addr: &str, chain_id: Option<&str>) -> Result<AccountRiskResponse, anyhow::Error> {
        let url = format!("{}/address_security/{}", BASE_URL, addr);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.clone().unwrap_or("None".to_string()))
            .query(&[("chain_id", chain_id.unwrap_or("None"))])
            .send()
            .await?
            .json::<AccountRiskResponse>()
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

    
    pub async fn approval_security_v2(&self, erc: V2ApprovalERC, chain_id: &str, address: &str) -> Result<V2ApprovalResponse, anyhow::Error> {
        
        let base_url = match erc {
            V2ApprovalERC::ERC20 => "https://api.gopluslabs.io/api/v2/token_approval_security",
            V2ApprovalERC::ERC721 => "https://api.gopluslabs.io/api/v2/nft721_approval_security",
            V2ApprovalERC::ERC1155 => "https://api.gopluslabs.io/api/v2/nft1155_approval_security/",
        };
        let url = format!("{}/{}", base_url, chain_id);
        
        Ok(self.inner.get(url)
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
            .query(&[("addresses", address)])
            .send()
            .await?
            .json::<V2ApprovalResponse>()
            .await?)

        
        

    }

    /// Decodes ABI input data for interacting with smart contracts.
    ///
    /// # Parameters
    /// - `chain_id`: Blockchain chain ID.
    /// - `data`: ABI data to decode.
    /// - `contract_addr`: Optional contract address.
    /// - `signer`: Optional signer.
    /// - `txn_type`: Optional transaction type.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.abi_decode(
    ///     "56", 
    ///     "0xa9059cbb00000000000000000000000055d398326f99059ff775485246999027b319795500000000000000000000000000000000000000000000000acc749097d9d00000", 
    ///     Some("0x55d398326f99059ff775485246999027b3197955"),
    ///     // None,
    ///     None, 
    ///     None
    /// ).await?;
    /// ```
    /// Parameters and response fields in depth [here](https://docs.gopluslabs.io/reference/response-details-4)
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
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
            .json(&params)
            .send()
            .await?
            .json::<AbiDecodeResponse>()
            .await?)
    }

    /// Evaluates NFT risk for a specific contract address on a blockchain.
    ///
    /// # Parameters
    /// - `chain_id`: Blockchain chain ID.
    /// - `contract_addr`: Contract address.
    /// - `token_id`: Optional token ID.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.nft_risk("1", "0x...", Some("123")).await?;
    /// let nft_risk: NftRisk = response.result;
    /// ```
    /// 
    /// Response fields explained in depth [here](https://docs.gopluslabs.io/reference/response-details-5)
    pub async fn nft_risk(&self, chain_id: &str, contract_addr: &str, token_id: Option<&str>) -> Result<NftRiskResponse, anyhow::Error> {
        let url = format!("{}/nft_security/{}",BASE_URL, chain_id);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
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
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
            .query(&[("url", dapp_url)])
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    /// Analyzes phishing risks for a given site URL.
    ///
    /// # Parameters
    /// - `site_url`: URL of the site to check.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.phishing_site_risk("go-ethdenver.com").await?;
    /// ```
    /// Response fields in depth [here](https://docs.gopluslabs.io/reference/phishingsiteusingget)
    pub async fn phishing_site_risk(&self, site_url: &str) -> Result<PhishingSiteResponse, anyhow::Error> {
        let url = format!("{}/phishing_site", BASE_URL);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
            .query(&[("url", site_url)])
            .send()
            .await?
            .json::<PhishingSiteResponse>()
            .await?)
    }
    
    /// Assesses the risk of a rug pull for a contract on a specific blockchain.
    ///
    /// # Parameters
    /// - `chain_id`: Blockchain chain ID.
    /// - `contract_addr`: Contract address.
    ///
    /// # Example Usage
    /// ```
    /// let session = Session::new();
    /// let response = session.rug_pull_risk("1", "0x6B175474E89094C44Da98b954EedeAC495271d0F").await?;
    /// ```
    /// Response fields in depth [here](https://docs.gopluslabs.io/reference/response-details-7)
    pub async fn rug_pull_risk(&self, chain_id: &str, contract_addr: &str) -> Result<RugPullRiskResponse, anyhow::Error> {
        let url = format!("{}/rugpull_detecting/{}", BASE_URL, chain_id);

        Ok(self.inner.get(url)
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
            .query(&[("contract_addresses", contract_addr)])
            .send()
            .await?
            .json::<RugPullRiskResponse>()
            .await?)
    }

    /// Obtains an access token using SHA-1 signature method.
    ///
    /// # Sign Method
    /// Concatenate `app_key`, `time`, and `app_secret` in turn, and apply SHA-1 hashing.
    /// 
    /// `time` should be +- 1000s around the current timestamp
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
    /// instance.get_access_token("mBOMg20QW11BbtyH4Zh0", "7293d385b9225b3c3f232b76ba97255d0e21063e", 1647847498).await?;
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
            .header("access_token", self.access_token.as_ref().unwrap_or(&"None".to_string()))
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