use near_sdk::{AccountId, PublicKey, ext_contract, PromiseOrValue};

#[ext_contract(linkdrop_contract)]
pub trait LinkdropContract {
    fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> PromiseOrValue<bool>;
}

// 宏展开的跨合约调用相关代码，不需要编写
// pub mod linkdrop_contract {
//
// }
