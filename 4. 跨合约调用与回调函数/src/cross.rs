use near_sdk::{AccountId, PublicKey, ext_contract, PromiseOrValue};

// 声明 linkdrop 合约的接口
// `#[ext_contract]` 宏如果不提供名称参数，则默认使用 trait 名的下划线格式
#[ext_contract(linkdrop_contract)]
pub trait LinkdropContract {
    fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> PromiseOrValue<bool>;
}

// 宏展开的跨合约调用相关代码，不需要编写
// pub mod linkdrop_contract {
//
// }
