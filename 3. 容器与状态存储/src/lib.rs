use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    // 来自 `near_sdk::store`
    // 该容器内的数据与容器本身分开存储，容器本身是根结构的一部分，但内部数据是独立的存储记录
    descriptions: LookupMap<AccountId, String>,
}

// 所有的 `near_sdk::store` 或 `near_sdk::collections` 容器在初始化的时候都需要唯一的 storage key
// 枚举类型恰好可以在序列化时保证唯一性
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Descriptions, // 以 0_u8 的方式 borsh 序列化
    OtherKey,     // 以 1_u8 的方式 borsh 序列化
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            descriptions: LookupMap::new(StorageKey::Descriptions),
        }
    }

    pub fn set_account_description(&mut self, account_id: AccountId, description: String) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.descriptions.insert(account_id, description);
    }

    pub fn get_account_description(&self, account_id: AccountId) -> Option<&String> {
        self.descriptions.get(&account_id)
    }
}
