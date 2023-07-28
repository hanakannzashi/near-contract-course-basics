use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use std::collections::HashMap;

#[near_bindgen] // 定义合约根结构
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] // 实现 borsh 序列化，实现不可用的 `default` 方法以通过编译
pub struct Contract {
    owner_id: AccountId,                      // 合约所有者
    descriptions: HashMap<AccountId, String>, // 账户及其描述信息。注: `std::collections` 作为容器不是最好的选择，因为容器本身与内部数据都存储在相同的记录中。此处仅用于教学目的
}

// impl Default for Contract {
//     fn default() -> Self {
//         todo!()
//     }
// }

#[near_bindgen] // 定义合约方法
impl Contract {
    #[init] // 标记合约初始化方法
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            descriptions: HashMap::new(),
        }
    }

    // Change 方法
    pub fn set_account_description(&mut self, account_id: AccountId, description: String) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.descriptions.insert(account_id, description);
    }

    // View 方法
    pub fn get_account_description(&self, account_id: AccountId) -> Option<&String> {
        self.descriptions.get(&account_id)
    }
}
