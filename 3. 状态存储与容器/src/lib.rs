use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    // 来自 `near_sdk::store`
    // 该容器内的数据与容器本身分开存储, 容器本身是根结构的一部分, 但内部数据是独立的存储记录
    descriptions: LookupMap<AccountId, String>,
}

// near sdk 提供的容器在初始化的时候都需要唯一的 storage key
// 可以使用 `#[derive(BorshStorageKey)]` 宏来获取 storage key. 它将枚举值按顺序以 `u8` 的方式进行 borsh 序列化, 最多可以得到 256 种不同的 storage key
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    // 以 0u8 的方式 borsh 序列化
    Descriptions,

    // 以 1u8 的方式 borsh 序列化, 该值在本合约中没有被使用, 仅用于教学目的
    #[allow(unused)]
    OtherKey,
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
