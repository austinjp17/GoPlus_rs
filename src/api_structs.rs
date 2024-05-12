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
pub struct AccountResponse {
    code: u32,
    message: String,
    result: AccountResult,
}

/// Data on the security and reliability of a given account
/// 
/// Contract address not returned if address passed without chain_id
#[derive(Deserialize, Debug, Clone)]
pub struct AccountResult {
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