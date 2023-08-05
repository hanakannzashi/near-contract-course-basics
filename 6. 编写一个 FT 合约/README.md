# 第六章 编写一个 FT 合约
FT 即 fungible token 同质化通证. 如同以太坊的 [ERC20](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md) 一样, NEAR 也有自己的 FT 标准 [NEP141](https://github.com/near/NEPs/blob/master/neps/nep-0141.md)

## FT 标准实现
near sdk 实现了标准的 FT, 实现代码在 `near-contract-standards` 库
```toml
[dependencies]
near-sdk = "4.1.1"
near-contract-standards = "4.1.1"
```

其中的 `FungibleToken` 类型即 FT 标准实现, 我们可以将其作为合约的一个字段, 然后把它的实现封装成合约方法.
`FungibleToken` 不仅实现了 NEP141, 还实现了 [NEP145](https://github.com/near/NEPs/blob/master/neps/nep-0145.md) 和 [NEP148](https://github.com/near/NEPs/blob/master/neps/nep-0148.md)

### NEP141
```rust
pub trait FungibleTokenCore {
    // 给普通账户转账
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    
    // 给合约账户转账, 以触发合约相关逻辑, 返回值的含义是实际转账的 FT 数量
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
    // 转账的回调函数, 返回值的含义是实际转账的 FT 数量
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128;
}
```

near sdk 提供了 `impl_fungible_token_core` 宏来快速给合约实现上述接口

### NEP 145
```rust
pub trait StorageManagement {
    // 注册 FT 持有者信息并支付存储费
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
    
    // 提取用户已支付的存储费
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance;
    
    // 注销 FT 持有者信息并返还存储费
    fn storage_unregister(&mut self, force: Option<bool>) -> bool;

    // 查询合约对单个用户需要的存储费范围
    fn storage_balance_bounds(&self) -> StorageBalanceBounds;

    // 查询用户支付的存储费
    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance>;
}
```

near sdk 提供了 `impl_fungible_token_storage` 宏来快速给合约实现上述接口

标准 FT 实现了存储管理是为了能够让合约不会因存储费不足而无法运行. 当用户持有 FT 的时候, 用户的信息会被记录在合约里, 这会占用存储费, 标准实现让用户自己支付存储费, 以避免合约存储费不足

### NEP148
```rust
pub trait FungibleTokenMetadataProvider {
    // FT 的详情
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}
```

near sdk 没有提供上述接口的实现, 别忘了自己手动给合约实现一下

## Mint 和 Burn
mint 和 burn 不是标准的操作, 因此我们需要自己实现. 需要注意的是 mint 的时候对于没有注册 FT 持有者信息的用户, 需要先注册再 mint.
也可以让用户自己从合约外部调用 `storage_deposit` 进行注册并支付存储费, 取决于开发者想怎么实现.

## 接收合约
如果一个合约需要接收用户转账的 FT, 则该合约需要实现 `ft_on_transfer` 来触发合约相关操作
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
