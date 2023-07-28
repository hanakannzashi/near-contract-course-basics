mod cross;

use crate::cross::linkdrop_contract;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{
    env, log, near_bindgen, serde_json, AccountId, Gas, GasWeight, PanicOnDefault, Promise,
    PromiseError, PromiseOrValue, PublicKey,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // linkdrop 合约地址
    linkdrop_contract_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(linkdrop_contract_id: AccountId) -> Self {
        Self {
            linkdrop_contract_id,
        }
    }

    // 通过 linkdrop 合约创建一个子账户，需要支付一定的 NEAR 作为初始余额，如果创建失败，则退回这笔费用
    // `#[payable]` 标注该方法在调用时接受附带的 NEAR
    #[payable]
    pub fn create_account_by_linkdrop(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<()> {
        let amount = env::attached_deposit();
        // `U128` 是 `u128` 的封装类型，使用 `String` 的 json 序列化方式
        // near sdk 默认使用 json 序列化作为外部传参方式，包括跨合约调用，使用封装类型避免大数产生精度丢失
        let wrapped_amount = U128(amount);

        linkdrop_contract::ext(self.linkdrop_contract_id.clone())                          // ext 是一个固定的方法
            .with_attached_deposit(amount)                                                 // 附带 NEAR 用于创建账户
            .create_account(new_account_id, new_public_key)                                // 创建调用 create_account 的 Promise，调用逻辑在当前区块不执行
            .then(
                Self::ext(env::current_account_id())                                       // ext 是一个固定的方法
                    .resolve_create_account(env::predecessor_account_id(), wrapped_amount) // 创建调用 resolve_create_account 的 Promise，调用逻辑在当前区块不执行
            ).into()
    }

    // 以下代码使用 Promise 直接编写，与上面的代码等价
    #[payable]
    pub fn create_account_by_linkdrop_use_promise_directly(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<()> {
        let amount = env::attached_deposit();
        let wrapped_amount = U128(amount);

        #[derive(Serialize)]
        #[serde(crate = "near_sdk::serde")]
        struct CreateAccountArgs {
            new_account_id: AccountId,
            new_public_key: PublicKey,
        }

        #[derive(Serialize)]
        #[serde(crate = "near_sdk::serde")]
        struct ResolveCreateAccountArgs {
            payer_id: AccountId,
            amount: U128,
        }

        Promise::new(self.linkdrop_contract_id.clone())
            .function_call_weight(
                "create_account".to_string(),
                serde_json::to_vec(&CreateAccountArgs {
                    new_account_id,
                    new_public_key,
                })
                .unwrap(),
                amount,       // 附带 NEAR 用于创建账户
                Gas(0),       // 不分配固定的 gas
                GasWeight(1), // 剩余 gas 分配时占 1 份
            )
            .then(
                Promise::new(env::current_account_id()).function_call_weight(
                    "resolve_create_account".to_string(),
                    serde_json::to_vec(&ResolveCreateAccountArgs {
                        payer_id: env::predecessor_account_id(),
                        amount: wrapped_amount,
                    })
                    .unwrap(),
                    0,
                    Gas(0),       // 不分配固定的 gas
                    GasWeight(1), // 剩余 gas 分配时占 1 份
                ),
            )
            .into()
    }

    pub fn resolve_create_account(
        &mut self,
        payer_id: AccountId,
        amount: U128,
        #[callback_result] is_success: Result<bool, PromiseError>,
    ) {
        if is_success.unwrap_or(false) {
            log!("Account is successfully created.");
        } else {
            log!("Fail to create account, refund the money.");
            Promise::new(payer_id).transfer(amount.0);
        }
    }
}
