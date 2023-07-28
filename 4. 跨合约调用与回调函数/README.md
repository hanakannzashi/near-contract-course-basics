# 第四章 跨合约调用与回调函数
NEAR 是一条异步链, 一笔交易会被拆分为一个或多个 Receipt 在不同的区块执行.
Receipt 在 near sdk 中被抽象为 `Promsie`, 发起跨合约调用就是创建一个 `Promise`, 调用逻辑在当前方法执行的区块不会执行, 而是在之后的区块中异步执行

由于跨了区块, 当发生 panic 时交易无法完全回滚, 只能回滚发生错误的那个 Receipt, 因此我们往往需要对调用结果进行回调, 以保证合约状态一致性

near sdk 针对跨合约调用提供了更高级的 API, 该 API 本质上是对 Promise 的封装

## 跨合约调用示例
该合约可以通过调用 linkdrop 合约提供的 `create_account` 方法创建一个子账户, 用户需要支付一定的 NEAR 作为初始余额, 如果创建失败, 则退回这笔费用

1. 在 [cross.rs](./src/cross.rs) 中声明 linkdrop 合约方法的接口
   ```rust
   #[ext_contract(linkdrop_contract)]
   pub trait LinkdropContract {
       fn create_account(&mut self, new_account_id: AccountId, new_public_key: PublicKey) -> PromiseOrValue<bool>;
   }
   ```
   该接口需要用宏 `#[ext_contract(linkdrop_contract)]` 标注，当宏展开时会在当前模块生成一个名为 linkdrop_contract 的 `pub mod`，里面包含跨合约调用相关代码
2. 在 [lib.rs](./src/lib.rs) 中实现回调函数，主要是创建失败后的退款逻辑
    ```rust
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
    ```
    如果被回调的方法有返回值，我们可以使用 `#[callback_result]` 来获取返回值，在本例中 linkdrop 合约的 `create_account` 方法返回 `bool` 类型，使用 `Result` 包裹是因为被回调的方法可能会 panic，此时我们无法拿到返回值
3. 在 [lib.rs](./src/lib.rs) 中编写完整的跨合约调用逻辑
   ```rust
   use crate::cross::linkdrop_contract;
   ```
   
   ```rust
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
   ```
   该方法需要用户支付 NEAR 作为被创建账户的初始余额，因此需要使用 `#[payable]` 标注。回调函数可以使用 `Self::ext` 直接调用，也可以像调用 linkdrop 合约方法一样，先声明接口，再使用宏展开的模块去调用
4. 在本例中对于跨合约调用的两个方法 `create_account` 和 `resolve_create_account` 并没有手动分配 gas，默认将多余的 gas 平均分配。我们可以使用 `with_static_gas` 或 `with_unused_gas_weight` 来手动分配 gas
   ```rust
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
           .with_unused_gas_weight(1)                              // 剩余 gas 分配时占 1 份，1 是默认值
           .create_account(new_account_id, new_public_key)
           .then(
               Self::ext(env::current_account_id())
                   .with_static_gas(Gas(20_000_000_000_000))       // 分配固定的 20T gas
                   .with_unused_gas_weight(0)                      // 不参与剩余 gas 分配
                   .resolve_create_account(env::predecessor_account_id(), wrapped_amount)
           ).into()
   }
   ```
