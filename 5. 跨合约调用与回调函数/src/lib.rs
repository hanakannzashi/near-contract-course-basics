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
    // Linkdrop 合约地址
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
}

#[near_bindgen]
impl Contract {
    // 通过 Linkdrop 合约创建一个子账户, 需要支付一定的 NEAR 作为初始余额, 如果创建失败. 则退回这笔费用
    // 该方法使用高级 API 编写, 不直接使用 `Promise`
    #[payable] // 标记该方法在调用时接受附带的 NEAR
    pub fn create_account_by_linkdrop(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<()> {
        let amount = env::attached_deposit();
        // `U128` 是 `u128` 的封装类型, 使用 `String` 的 JSON 序列化方式, 避免大数在序列化之后产生精度丢失
        let amount = U128(amount);

        // `ext` 是一个固定的方法
        linkdrop_contract::ext(self.linkdrop_contract_id.clone())
            // 附带 NEAR 用于创建账户
            .with_attached_deposit(amount.0)
            // 创建调用 `create_account` 的 `Promise`, 调用逻辑在当前区块不执行
            .create_account(new_account_id, new_public_key)
            .then(
                // `ext` 是一个固定的方法, 除了使用 `Self::ext` 之外, 也可以像调用 Linkdrop 合约一样先声明接口, 再通过模块进行调用
                Self::ext(env::current_account_id())
                    // 创建调用 `resolve_create_account` 的 `Promise`, 调用逻辑在当前区块不执行
                    .resolve_create_account(env::predecessor_account_id(), amount),
            )
            .into()
    }
}

#[near_bindgen]
impl Contract {
    // 通过 Linkdrop 合约创建一个子账户, 需要支付一定的 NEAR 作为初始余额, 如果创建失败. 则退回这笔费用
    // 该方法直接使用 `Promise` 编写, 不使用高级 API, 代码量更大
    #[payable]
    pub fn create_account_by_linkdrop_using_promise(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<()> {
        let amount = env::attached_deposit();
        let amount = U128(amount);

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
                amount.0,
                Gas(0),
                GasWeight(1),
            )
            .then(
                Promise::new(env::current_account_id()).function_call_weight(
                    "resolve_create_account".to_string(),
                    serde_json::to_vec(&ResolveCreateAccountArgs {
                        payer_id: env::predecessor_account_id(),
                        amount,
                    })
                    .unwrap(),
                    0,
                    Gas(0),
                    GasWeight(1),
                ),
            )
            .into()
    }
}

#[near_bindgen]
impl Contract {
    #[private] // 标记该方法只能由合约自己调用
    pub fn resolve_create_account(
        &mut self,
        payer_id: AccountId,
        amount: U128,
        // 如果被回调的方法有返回值, 可以使用 `#[callback_result]` 来获取返回值
        #[callback_result] is_success: Result<bool, PromiseError>,
    ) {
        if is_success.unwrap_or(false) {
            log!("Account is successfully created.");
        } else {
            log!("Failed to create account, refund the money.");
            Promise::new(payer_id).transfer(amount.0);
        }
    }
}
