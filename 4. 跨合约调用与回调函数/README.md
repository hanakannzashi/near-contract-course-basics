# 第四章 跨合约调用与回调函数
NEAR 是一条异步链, 一笔交易会被拆分为一个或多个 Receipt 在不同的区块执行.
Receipt 在 near sdk 中被抽象为 `Promsie`, 发起跨合约调用就是创建一个包含 FunctionCall Action 的 `Promise`, 调用逻辑在当前方法执行的区块不会执行, 而是在之后的区块中异步执行

由于 Receipt 跨了区块, 当交易失败时无法完全回滚, 只会回滚发生错误的那个 Receipt, 因此我们往往需要对调用结果进行回调, 以保证合约状态一致性

near sdk 针对跨合约调用提供了高级 API, 该 API 本质上是对 Promise 的封装

## 跨合约调用示例 (使用高级 API)
该合约可以通过调用 linkdrop 合约提供的 `create_account` 方法创建一个子账户, 用户需要支付一定的 NEAR 作为初始余额, 如果创建失败, 则退回这笔费用

1. 在 [cross.rs](./src/cross.rs) 中声明 linkdrop 合约方法的接口
   ```rust
   #[ext_contract(linkdrop_contract)]
   pub trait LinkdropContract {
       fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> PromiseOrValue<bool>;
   }
   ```
2. 在 [lib.rs](./src/lib.rs) 中实现回调函数, 主要实现账户创建失败后的退款逻辑
   ```rust
   #[near_bindgen]
   impl Contract {
       pub fn resolve_create_account(
           &mut self,
           payer_id: AccountId,
           amount: U128,
           #[callback_result] is_success: Result<bool, PromiseError>
       ) {
           if is_success.unwrap_or(false) {
               log!("Account is successfully created.");
           } else {
               log!("Fail to create account, refund the money.");
               Promise::new(payer_id).transfer(amount.0);
           }
       }
   }
   ```
3. 在 [lib.rs](./src/lib.rs) 中编写完整的跨合约调用逻辑
   ```rust
   use crate::cross::linkdrop_contract;
   ```
   
   ```rust
   #[near_bindgen]
   impl Contract {
       #[payable]
       pub fn create_account_by_linkdrop(
           &mut self,
           new_account_id: AccountId,
           new_public_key: PublicKey
       ) -> PromiseOrValue<()> {
           let amount = env::attached_deposit();
           let wrapped_amount = U128(amount);

           linkdrop_contract::ext(self.linkdrop_contract_id.clone())
               .with_attached_deposit(amount)
               .create_account(new_account_id, new_public_key)
               .then(
                   Self::ext(env::current_account_id())
                       .resolve_create_account(env::predecessor_account_id(), wrapped_amount)
               ).into()
       }
   }
   ```

## 手动分配 gas
跨合约调用时默认将剩余 gas 平均分配给所有调用, 可以通过 `with_static_gas` 和 `with_unused_gas_weight` 手动分配 gas, 具体可查看示例代码

## 直接使用 Promise 的版本
以上代码使用高级 API 编写, 示例中提供了直接使用 `Promise` 的版本, 可自行查看.
只有跨合约调用 (即单个 FunctionCall Action) 才能使用高级 API, 其他异步操作如转账 NEAR 只能直接使用 `Promise`
