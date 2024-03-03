use near_sdk::{ext_contract, AccountId, PromiseOrValue, PublicKey};

// 声明 Linkdrop 合约的接口
#[ext_contract(linkdrop_contract)]
pub trait LinkdropContract {
    fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<bool>;
}

// 宏展开后会生成一个 `pub mod linkdrop_contract`, 包含对 `Promise` 的封装
// 该代码自动生成, 无需编写
// pub mod linkdrop_contract {
//
// }
