This repository contains a Rust API wrapper for interacting with GoPlusLabs services for risk metrics and analysis on tokens, smart contracts, and wallets across different chain ecosystems. 

## Requirements

- Rust Programming Language
- `reqwest` and `serde_json` crates for HTTP requests and JSON handling, respectively.
<!-- - Environment variables `GP_PUBLIC` and `GP_SECRET` must be set with your application keys. -->

## Getting Started

1. Add crate: `cargo add goplus_rs`

2. Set app keys as enviornment variables to get access code. (Currently doesn't affect usage but may in the future.)

    ```
    export GP_PUBLIC = $APP_PUBLIC_KEY$
    export GP_PUBLIC = $APP_PRIVATE_KEY$
    ```

3. Create persistant session
   ```
    use goplus_rs;
    let instance = goplus_rs::Session::new();
   ```



<!-- 
## API Methods

- **get_access_token()** 
    
    Retrieves an idenitifying session token
- supported_chains(): 
  
    Retrieves supported ecosystem chains.
- token_risk(chain_id, addr): 

    Fetches token risk data.
- address_risk(addr, chain_id): 
  
    Risk information and suspicious activity for an address.
- approval_security_v1(addr, chain_id): 
  
    Security approval risk information
- approval_security_v2(): 
    
    todo!()
- abi_decode( chain_id, data, ..): 
  
    Decode ABI data for contract interaction mappings
- nft_risk(chain_id, contract_addr, ..): 
    
    Risk metrics for a given nft contract
- dapp_risk_by_url(dapp_url): 
  
    TODO. App fails to be found.
- phishing_site_risk(site_url): 
  
    Analyzes phishing risks for a given site URL.
- rug_pull_risk(chain_id, contract_addr): 
    
      Assesses the risk of a rug pull for a contract. -->


