#![allow(dead_code)]
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportedChainsResponse {
    code: u32,
    message: String,
    result: Vec<Chain>
}


// Get available chains call result struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chain {
    name: String,
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenResponse {
    code: u32,
    message: String,
    result: std::collections::HashMap<String, TokenData>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct TokenData {
    anti_whale_modifiable: String,
    buy_tax: String,
    can_take_back_ownership: String,
    cannot_buy: String,
    cannot_sell_all: String,
    creator_address: String,
    creator_balance: String,
    creator_percent: String,
    dex: Vec<DexInfo>,
    external_call: String,
    hidden_owner: String,
    holder_count: String,
    holders: Vec<HolderInfo>,
    honeypot_with_same_creator: String,
    is_anti_whale: String,
    is_blacklisted: String,
    is_honeypot: String,
    is_in_dex: String,
    is_mintable: Option<String>,
    is_open_source: String,
    is_proxy: Option<String>,
    is_whitelisted: String,
    lp_holder_count: String,
    lp_holders: Vec<LpHolderInfo>,
    lp_total_supply: String,
    note: Option<String>,
    other_potential_risks: Option<String>,
    owner_address: Option<String>,
    owner_balance: Option<String>,
    owner_change_balance: Option<String>,
    owner_percent: Option<String>,
    personal_slippage_modifiable: Option<String>,
    selfdestruct: Option<String>,
    sell_tax: Option<String>,
    slippage_modifiable: String,
    token_name: String,
    token_symbol: String,
    total_supply: Option<String>,
    trading_cooldown: String,
    transfer_pausable: String,
    trust_list: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DexInfo {
    liquidity_type: String,
    name: String,
    liquidity: String,
    pair: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HolderInfo {
    address: String,
    tag: String,
    is_contract: u32,
    balance: String,
    percent: String,
    is_locked: u32,
    locked_detail: Option<Vec<LockedDetail>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LockedDetail {
    amount: String,
    end_time: String,
    opt_time: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LpHolderInfo {
    address: String,
    tag: String,
    value: Option<String>,
    is_contract: u32,
    balance: String,
    percent: String,
    nft_list: Option<Vec<NftInfo>>,
    is_locked: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NftInfo {
    nft_id: String,
    nft_percentage: String,
    amount: String,
    in_effect: String,
    value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountRiskResponse {
    code: u32,
    message: String,
    result: AccountRisk,
}

/// Data on the security and reliability of a given account
/// 
/// Contract address not returned if address passed without chain_id
#[derive(Deserialize, Debug, Clone)]
pub struct AccountRisk {
    blacklist_doubt: String,
    blackmail_activities: String,
    contract_address: Option<String>,
    cybercrime: String,
    darkweb_transactions: String,
    data_source: String,
    fake_kyc: String,
    financial_crime: String,
    honeypot_related_address: String,
    malicious_mining_activities: String,
    mixer: String,
    money_laundering: String,
    number_of_malicious_contracts_created: String,
    phishing_activities: String,
    sanctioned: String,
    stealing_attack: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ASresponse {
    code: u32,
    message: String,
    result: ASresult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ASresult {
    contract_name: Option<String>,
    creator_address: Option<String>,
    deployed_time: Option<u64>,  // Assuming it's a timestamp in seconds
    doubt_list: u32,
    is_contract: u32,
    is_open_source: Option<u32>,
    is_proxy: Option<u32>,
    malicious_behavior: Vec<String>,
    tag: Option<String>,
    trust_list: u32,
    contract_scan: ContractScan,
    risky_approval: RiskyApproval
}

#[derive(Deserialize, Debug, Clone)]
pub struct RiskyApproval {
    value: u8,
    risk: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractScan {
    owner: Option<Owner>,
    privilege_withdraw: Option<String>,
    withdraw_missing: Option<String>,
    blacklist: Option<String>,
    selfdestruct: Option<String>,
    approval_abuse: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Owner {
    owner_name: Option<String>,
    owner_address: Option<String>,
    owner_type: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct NftRiskResponse {
    code: u32,
    message: String,
    result: NftRisk,
}


#[derive(Deserialize, Debug, Clone)]
pub struct NftRisk {
    average_price_24h: f64,
    create_block_number: Option<u64>,
    creator_address: Option<String>,
    discord_url: Option<String>,
    github_url: Option<String>,
    highest_price: f64,
    lowest_price_24h: f64,
    malicious_nft_contract: u32,
    medium_url: Option<String>,
    metadata_frozen: Option<u8>,
    nft_address: String,
    nft_description: Option<String>,
    nft_erc: String,
    nft_items: u32,
    nft_name: String,
    nft_open_source: u8,
    nft_owner_number: u32,
    nft_proxy: Option<u8>,
    nft_symbol: Option<String>,
    nft_verified: u32,
    oversupply_minting: Option<u8>,
    privileged_burn: PrivilegedAction,
    privileged_minting: PrivilegedAction,
    red_check_mark: Option<()>,
    restricted_approval: Option<u8>,
    sales_24h: u32,
    same_nfts: Vec<SameNft>,
    self_destruct: PrivilegedAction,
    telegram_url: Option<String>,
    token_id: Option<String>,
    token_owner: Option<String>,
    total_volume: f64,
    traded_volume_24h: f64,
    transfer_without_approval: PrivilegedAction,
    trust_list: u32,
    twitter_url: Option<String>,
    website_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PrivilegedAction {
    owner_address: Option<String>,
    owner_type: Option<String>,
    value: Option<u8>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SameNft {
    create_block_number: u64,
    nft_address: String,
    nft_name: String,
    nft_owner_number: u32,
    nft_symbol: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhishingSiteResponse {
    code: u32,
    message: String,
    result: PhishingSiteResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhishingSiteResult {
    pub phishing_site: i32,
    pub website_contract_security: Vec<WebsiteContractSecurity>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebsiteContractSecurity {
    pub address_risk: Vec<String>,
    pub contract: String,
    pub is_contract: i32,
    pub is_open_source: i32,
    pub nft_risk: NftData,
    pub standard: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NftData {
    pub nft_open_source: i32,
    pub nft_proxy: i32,
    pub oversupply_minting: i32,
    pub privileged_burn: PrivilegedAction,
    pub privileged_minting: PrivilegedAction,
    pub restricted_approval: i32,
    pub self_destruct: PrivilegedAction,
    pub transfer_without_approval: PrivilegedAction,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RugPullRiskResponse {
    code: u32,
    message: String,
    result: RugPullResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RugPullResult {
    pub approval_abuse: i32,
    pub blacklist: i32,
    pub contract_name: String,
    pub is_open_source: i32,
    pub is_proxy: i32,
    pub owner: Option<Owner>,
    pub privilege_withdraw: i32,
    pub selfdestruct: i32,
    pub withdraw_missing: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccessCodeResponse {
    pub code: u32,
    pub message: String,
    pub result: Option<AccessCodeResult>
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccessCodeResult {
    pub access_token: String,
    pub expires_in: u64
}
#[derive(Deserialize, Debug, Clone)]
pub struct AbiDecodeResponse {
    pub code: u32,
    pub message: String,
    pub result: AbiDecodeResult,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AbiDecodeResult {
    pub contract_description: Option<String>,
    pub contract_name: String,
    pub data: Option<serde_json::Value>,
    pub malicious_contract: Option<u8>,
    pub method: String,
    pub params: Vec<Param>,
    pub risk: Option<String>,
    pub risky_signature: Option<i32>,
    pub signature_detail: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Param {
    pub address_info: Option<AddressInfo>,
    pub input: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub tuple: Option<serde_json::Value>,
    #[serde(rename = "struct")]
    pub struct_field: Option<serde_json::Value>
    
}

#[derive(Clone, Deserialize, Debug)]
pub struct AddressInfo {
    pub contract_name: String,
    pub is_contract: i32,
    pub malicious_address: i32,
    pub name: String,
    pub standard: String,
    pub symbol: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct V2ApprovalResponse {
    pub code: u32,
    pub message: String,
    pub result: Vec<V2ApprovalResult>
}

#[derive(Deserialize, Debug, Clone)]
pub struct V2ApprovalResult {
    pub approved_list: Vec<V2ApprovedContract>,
    pub balance: String,
    pub chain_id: String,
    pub decimals: u32,
    pub is_open_source: u8,
    pub malicious_address: u8,
    pub malicious_behavior: Option<Vec<String>>,
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct V2ApprovedContract {
    pub address_info: V2ApprovalAddressInfo,
    pub approved_amount: String,
    pub approved_contract: String,
    pub approved_time: u64,
    pub hash: String,
    pub initial_approval_hash: String,
    pub initial_approval_time: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct V2ApprovalAddressInfo {
    contract_name: Option<String>,
    creator_address: String,
    deployed_time: u64,
    doubt_list: u64,
    is_contract: u64,
    is_open_source: u64,
    malicious_behavior: Option<Vec<String>>, // Adjusted to Vec<String> if behavior is just a list of strings
    tag: Option<String>, // Optional to handle both Null and String cases
    trust_list: u64,
}