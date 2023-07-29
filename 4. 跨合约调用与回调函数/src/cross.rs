use near_sdk::{ext_contract, AccountId, PromiseOrValue, PublicKey};

// 声明 linkdrop 合约的接口
#[ext_contract(linkdrop_contract)] // 如果不提供名称参数，则默认使用 trait 名的下划线格式
pub trait LinkdropContract {
    fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<bool>; // 使用 `PromiseOrValue` 而不使用 `Promise` 作为返回值是为了让方法返回值类型的定义更加清晰
}

// 宏展开后会生成一个 `pub mod linkdrop_contract`, 包含对 `Promise` 的封装
// 该代码自动生成, 无需编写
// pub mod linkdrop_contract {
//
// }
