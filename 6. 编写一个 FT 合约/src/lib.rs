use near_contract_standards::fungible_token::events::{FtBurn, FtMint};
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::{impl_fungible_token_core, impl_fungible_token_storage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens: FungibleToken,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            tokens: FungibleToken::new(StorageKey::FungibleToken),
        }
    }

    // 合约所有者能为任意用户 mint 指定数量的 FT
    pub fn mint(&mut self, account_id: AccountId, amount: U128, memo: Option<String>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.internal_mint(&account_id, amount.0, memo);
    }

    // 合约所有者能为任意用户 burn 指定数量的 FT
    pub fn burn(&mut self, account_id: AccountId, amount: U128, memo: Option<String>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.internal_burn(&account_id, amount.0, memo);
    }
}

// 为合约实现 NEP141 接口
// ft_transfer
// ft_transfer_call
// ft_total_supply
// ft_balance_of
// ft_resolve_transfer
impl_fungible_token_core!(Contract, tokens);

// 为合约实现 NEP145 接口
// storage_deposit
// storage_withdraw
// storage_unregister
// storage_balance_bounds
// storage_balance_of
impl_fungible_token_storage!(Contract, tokens);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Hello Fungible Token".to_string(),
            symbol: "HFT".to_string(),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

// ------------------------------------- 合约内部方法 ------------------------------------------------

impl Contract {
    pub(crate) fn internal_mint(
        &mut self,
        account_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        // 注册 FT 持有者信息
        if !self.tokens.accounts.contains_key(account_id) {
            self.tokens.internal_register_account(account_id);
        }

        // mint
        self.tokens.internal_deposit(account_id, amount);

        // 打印标准 log
        FtMint {
            owner_id: account_id,
            amount: &U128(amount),
            memo: memo.as_deref(),
        }
        .emit();
    }

    pub(crate) fn internal_burn(
        &mut self,
        account_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        // burn
        self.tokens.internal_withdraw(account_id, amount);

        // 打印标准 log
        FtBurn {
            owner_id: account_id,
            amount: &U128(amount),
            memo: memo.as_deref(),
        }
        .emit();
    }
}
