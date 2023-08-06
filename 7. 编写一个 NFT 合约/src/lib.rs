use near_contract_standards::non_fungible_token::events::{NftBurn, NftMint};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_contract_standards::{
    impl_non_fungible_token_approval, impl_non_fungible_token_core,
    impl_non_fungible_token_enumeration,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens: NonFungibleToken,

    // 使用全局自增 id 作为 NFT id
    unique_id: u64,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            unique_id: 0,
        }
    }

    // 合约所有者能为任意用户 mint NFT
    pub fn mint(&mut self, account_id: AccountId, metadata: TokenMetadata, memo: Option<String>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        let token_id = self.next_id().to_string();
        self.internal_mint(&account_id, &token_id, &metadata, memo);
    }

    // 合约所有者能为任意用户 burn NFT
    pub fn burn(&mut self, account_id: AccountId, token_id: TokenId, memo: Option<String>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.internal_burn(&account_id, &token_id, memo);
    }
}

// 为合约实现 NEP171
// nft_transfer
// nft_transfer_call
// nft_token
// nft_resolve_transfer
impl_non_fungible_token_core!(Contract, tokens);

// 为合约实现 NEP178
// nft_approve
// nft_revoke
// nft_revoke_all
// nft_is_approved
impl_non_fungible_token_approval!(Contract, tokens);

// 为合约实现 NEP181
// nft_total_supply
// nft_tokens
// nft_supply_for_owner
// nft_tokens_for_owner
impl_non_fungible_token_enumeration!(Contract, tokens);

// 为合约实现 NEP177
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Hello Non Fungible Token".to_string(),
            symbol: "HelloNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }
}

// ------------------------------------- 合约内部方法 ------------------------------------------------

impl Contract {
    pub(crate) fn next_id(&mut self) -> u64 {
        self.unique_id += 1;
        self.unique_id
    }

    pub(crate) fn internal_mint(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
        metadata: &TokenMetadata,
        memo: Option<String>,
    ) {
        // 添加 token_id -> token_owner_id 映射
        self.tokens.owner_by_id.insert(token_id, account_id);

        // 更新或添加 token_owner_id -> token_ids 映射
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut owner_tokens = tokens_per_owner.get(account_id).unwrap_or_else(|| {
                UnorderedSet::new(
                    near_contract_standards::non_fungible_token::core::StorageKey::TokensPerOwner {
                        account_hash: env::sha256(account_id.as_bytes()),
                    },
                )
            });
            owner_tokens.insert(token_id);
            tokens_per_owner.insert(account_id, &owner_tokens);
        }

        // 添加 token_id -> token_metadata 映射
        if let Some(token_metadata_by_id) = &mut self.tokens.token_metadata_by_id {
            token_metadata_by_id.insert(token_id, metadata);
        }

        // 打印标准 log
        NftMint {
            owner_id: account_id,
            token_ids: &[token_id],
            memo: memo.as_deref(),
        }
        .emit();
    }

    pub(crate) fn internal_burn(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
        memo: Option<String>,
    ) {
        // 移除 token_id -> token_owner_id 映射
        self.tokens.owner_by_id.remove(token_id);

        // 更新或移除 token_owner_id -> token_ids 映射
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            if let Some(mut owner_tokens) = tokens_per_owner.remove(account_id) {
                owner_tokens.remove(token_id);
                if !owner_tokens.is_empty() {
                    tokens_per_owner.insert(account_id, &owner_tokens);
                }
            }
        };

        // 移除 token_id -> token_metadata 映射
        if let Some(token_metadata_by_id) = &mut self.tokens.token_metadata_by_id {
            token_metadata_by_id.remove(token_id);
        }

        // 移除 token_id -> approval_ids 映射
        if let Some(approvals_by_id) = &mut self.tokens.approvals_by_id {
            approvals_by_id.remove(token_id);
        }

        // 移除 token_id -> next_approval_id 映射
        if let Some(next_approval_id_by_id) = &mut self.tokens.next_approval_id_by_id {
            next_approval_id_by_id.remove(token_id);
        }

        // 打印标准 log
        NftBurn {
            owner_id: account_id,
            token_ids: &[token_id],
            authorized_id: Some(account_id),
            memo: memo.as_deref(),
        }
        .emit();
    }
}

#[cfg(test)]
mod test {
    use crate::Contract;
    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;

    use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
    use near_contract_standards::non_fungible_token::TokenId;

    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, ONE_NEAR, ONE_YOCTO};

    fn owner() -> AccountId {
        "owner.near".parse().unwrap()
    }

    fn alice() -> AccountId {
        "alice.near".parse().unwrap()
    }

    fn bob() -> AccountId {
        "bob.near".parse().unwrap()
    }

    fn token(token_id: TokenId) -> TokenMetadata {
        TokenMetadata {
            title: Some(format!("HelloNFT #{}", token_id)),
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_mint_transfer_burn() {
        let mut contract = Contract::init(owner());

        let token_id_1 = "1".to_string();
        let token_1 = token(token_id_1.clone());
        let token_id_2 = "2".to_string();
        let token_2 = token(token_id_2.clone());

        // --------------------------------- 给 Bob mint NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.mint(bob(), token_1, None);
        contract.mint(bob(), token_2, None);

        assert_eq!(
            contract.nft_token(token_id_1.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(
            contract.nft_token(token_id_2.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(contract.nft_total_supply(), U128(2));

        // -------------------------------- Bob 给 Alice 转 NFT -------------------------------------

        // `nft_transfer` 调用需要附加 1 yocto NEAR
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(bob())
            .attached_deposit(ONE_YOCTO)
            .build());

        contract.nft_transfer(alice(), token_id_1.clone(), None, None);

        assert_eq!(
            contract.nft_token(token_id_1.clone()).unwrap().owner_id,
            alice()
        );
        assert_eq!(
            contract.nft_token(token_id_2.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(contract.nft_total_supply(), U128(2));

        // ---------------------------------- 销毁 Bob 的 NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.burn(bob(), token_id_2.clone(), None);

        assert_eq!(contract.nft_token(token_id_1).unwrap().owner_id, alice());
        assert!(contract.nft_token(token_id_2).is_none());
        assert_eq!(contract.nft_total_supply(), U128(1));
    }

    // 当用户把 NFT 授权给别的账户时, 别的账户就有权转移这个 NFT, 该功能通常用于在 NFT 市场挂单
    // 转移 NFT 时, 如果没有传 `approval_id` 参数, 则不校验该值
    // 否则必须保证传入的参数和实际值一致才能转移成功, 该校验是为了保护挂单中的 NFT 的安全性
    #[test]
    fn test_approve_transfer() {
        let mut contract = Contract::init(owner());

        let token_id = "1".to_string();
        let token = token(token_id.clone());

        // --------------------------------- 给 Bob mint NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.mint(bob(), token, None);

        assert_eq!(
            contract.nft_token(token_id.clone()).unwrap().owner_id,
            bob()
        );

        // ------------------------------- Bob 授权 NFT 给 Alice ------------------------------------

        // `nft_approve` 需要附加一些 NEAR 作为授权账户 id 的存储费, 多余的部分会自动退还
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(bob())
            .attached_deposit(ONE_NEAR / 100) // 附加 0.01 NEAR
            .build());

        contract.nft_approve(token_id.clone(), alice(), None);

        assert!(contract.nft_is_approved(token_id.clone(), alice(), None));

        // ---------------------------- Alice 通过授权把 Bob 的NFT 转给自己 ---------------------------

        // `nft_transfer` 调用需要附加 1 yocto NEAR
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(alice())
            .attached_deposit(ONE_YOCTO)
            .build());

        contract.nft_transfer(alice(), token_id.clone(), None, None);

        assert_eq!(
            contract.nft_token(token_id.clone()).unwrap().owner_id,
            alice()
        );
        assert!(!contract.nft_is_approved(token_id, alice(), None));
    }
}
