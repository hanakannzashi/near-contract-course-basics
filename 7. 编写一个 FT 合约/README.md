# 编写一个 FT 合约
FT 即 Fungible Token 同质化通证

NEAR 的 FT 标准
* [NEP141](https://github.com/near/NEPs/blob/master/neps/nep-0141.md)
* [NEP148](https://github.com/near/NEPs/blob/master/neps/nep-0148.md)

## FT 标准实现
NEAR SDK 实现了标准的 FT, 需要额外引入 `near-contract-standards`
```toml
[dependencies]
near-sdk = "4.1.1"
near-contract-standards = "4.1.1"
```

其中的 `FungibleToken` 类型即 FT 标准实现, 我们可以将其作为合约的一个字段, 然后把它的实现封装成合约方法.
标准 FT 不仅实现了 NEP141 和 NEP148, 还实现了存储管理 [NEP145](https://github.com/near/NEPs/blob/master/neps/nep-0145.md)

### NEP141
```rust
pub trait FungibleTokenCore {
    // 给普通账户转账. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    
    // 给合约账户转账, 以触发合约相关逻辑, 返回值的含义是实际转账的 FT 数量. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
    
    // 查询 FT 总供应量
    fn ft_total_supply(&self) -> U128;

    // 查询账户持有 FT 的数量
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

pub trait FungibleTokenResolver {
    // `ft_transfer_call` 内部的回调函数
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128;
}
```

`near-contract-standards` 提供了 `impl_fungible_token_core` 宏来快速给合约实现上述接口

### NEP 145
```rust
pub trait StorageManagement {
    // 注册 FT 持有者信息并支付存储费. 调用该方法需要附加一定量的 NEAR 作为存储费
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
    
    // 提取用户已支付的存储费. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance;
    
    // 注销 FT 持有者信息并返还存储费. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn storage_unregister(&mut self, force: Option<bool>) -> bool;

    // 查询合约对单个用户需要的存储费范围
    fn storage_balance_bounds(&self) -> StorageBalanceBounds;

    // 查询用户支付的存储费, 对于未注册的用户返回 `None`
    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance>;
}
```

`near-contract-standards` 提供了 `impl_fungible_token_storage` 宏来快速给合约实现上述接口

### NEP148
```rust
pub trait FungibleTokenMetadataProvider {
    // FT 合约详情
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}
```

## Mint 和 Burn
mint 和 burn 不是标准的操作, 因此我们需要自己实现. 需要注意的是 mint 的时候对于没有注册 FT 持有者信息的用户, 需要先注册再 mint.
也可以让用户自己从合约外部调用 `storage_deposit` 进行注册并支付存储费, 取决于开发者想怎么实现

## 接收合约
如果一个合约需要感知到自己接收了用户转账的 FT, 则该合约需要实现 `ft_on_transfer` 来触发合约相关操作
```rust
pub trait FungibleTokenReceiver {
    // 接收合约的这个方法会被 `ft_transfer_call` 调用, 返回值的含义是退回的 FT 数量
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}
```
