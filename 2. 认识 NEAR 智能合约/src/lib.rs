use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use std::collections::HashMap;

#[near_bindgen] // 定义合约根结构, 一个项目中只能有一个根结构
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] // 实现 Borsh 序列化
pub struct Contract {
    // 合约所有者
    owner_id: AccountId,
    // 账户及其描述信息. 注: `std::collections` 作为容器不是最好的选择, 此处仅用于教学目的
    descriptions: HashMap<AccountId, String>,
}

#[near_bindgen] // 定义合约方法
impl Contract {
    #[init] // 标记合约初始化方法
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            descriptions: HashMap::new(),
        }
    }

    // Change 方法, 第一个参数为 `&mut self`
    pub fn set_account_description(&mut self, account_id: AccountId, description: String) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.descriptions.insert(account_id, description);
    }

    // View 方法, 第一个参数为 `&self`
    pub fn get_account_description(&self, account_id: AccountId) -> Option<&String> {
        self.descriptions.get(&account_id)
    }
}
