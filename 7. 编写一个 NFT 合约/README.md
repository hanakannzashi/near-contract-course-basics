# 第七章 编写一个 NFT 合约
NFT 即 non fungible token 非同质化通证

NEAR 的 NFT 标准
* [NEP171](https://github.com/near/NEPs/blob/master/neps/nep-0171.md)
* [NEP177](https://github.com/near/NEPs/blob/master/neps/nep-0177.md)
* [NEP178](https://github.com/near/NEPs/blob/master/neps/nep-0178.md)
* [NEP181](https://github.com/near/NEPs/blob/master/neps/nep-0181.md)

## NFT 标准实现
near-contract-standards 实现了标准的 NFT
```toml
[dependencies]
near-sdk = "4.1.1"
near-contract-standards = "4.1.1"
```

其中的 `NonFungibleToken` 类型即 NFT 标准实现, 我们可以将其作为合约的一个字段, 然后把它的实现封装成合约方法

### NEP171
```rust
pub trait NonFungibleTokenCore {
    // 给普通账户转移 NFT. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
    
    // 给合约账户转移 NFT, 以触发合约相关逻辑, 返回值的含义是转移是否成功. 调用该方法需要附加 1 yocto NEAR 以保证安全性
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;
    
    // 查询某个 NFT 详情
    fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}

pub trait NonFungibleTokenResolver {
    // `nft_transfer_call` 内部的回调函数
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool;
}
```

near-contract-standards 提供了 `impl_non_fungible_token_core` 宏来快速给合约实现上述接口

### NEP 178
```rust
pub trait NonFungibleTokenApproval {
    // 当用户把 NFT 授权给别的账户时, 别的账户就有权转移这个 NFT, 该功能通常用于在 NFT 市场挂单
    // 转移 NFT 时, 如果没有传 `approval_id` 参数, 则不校验该值
    // 否则必须保证传入的参数和实际值一致才能转移成功, 该校验是为了保护挂单中的 NFT 的安全性
    // 当 NFT 成功发生转移之后, 该 NFT 所有的授权都会被重置
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise>;
    
    // 取消某个 NFT 对某个账户的授权
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);
    
    // 取消某个 NFT 对所有账户的授权
    fn nft_revoke_all(&mut self, token_id: TokenId);
    
    // 判断 NFT 是否授权给某个账户
    // 如果没有传 `approval_id` 参数, 则不校验该值
    // 否则必须保证传入的参数和实际值一致才会返回 `true`
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;
}
```

near-contract-standards 提供了 `impl_non_fungible_token_approval` 宏来快速给合约实现上述接口

### NEP181
```rust
pub trait NonFungibleTokenEnumeration {
    // 查询 NFT 总供应量
    fn nft_total_supply(&self) -> U128;
    
    // 查询多个 NFT 详情
    fn nft_tokens(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token>;
    
    // 查询某个用户持有的 NFT 数量
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128;

    // 查询某个用户持有的多个 NFT 详情
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token>;
}
```

near-contract-standards 提供了 `impl_non_fungible_token_enumeration` 宏来快速给合约实现上述接口

### NEP 177
```rust
pub trait NonFungibleTokenMetadataProvider {
    // NFT 合约详情
    fn nft_metadata(&self) -> NFTContractMetadata;
}
```

## Mint 和 Burn
mint 和 burn 不是标准的操作, 因此我们需要自己实现. 需要注意的是 mint NFT 会大量占用存储, 需要关注合约中用于存储质押的 NEAR 是否足够.
标准 NFT 提供了一个 `internal_mint` 方法, 该方法需要调用 mint 的账户支付存储费, 如果合约不复杂也可以用这个方法来实现 mint 功能

## 接收合约
如果一个合约需要感知到自己接收了用户转账的 NFT, 则该合约需要实现 `nft_on_transfer` 来触发合约相关操作
```rust
pub trait NonFungibleTokenReceiver {
    // 接收合约的这个方法会被 `nft_transfer_call` 调用, 返回值的含义是是否需要退回 NFT
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool>;
}
```

## 接收授权合约
如果一个合约需要感知到自己被用户授权了 NFT, 则该合约需要实现 `nft_on_approve` 来触发合约相关操作
```rust
pub trait NonFungibleTokenApprovalReceiver {
    // 授权合约的这个方法会被 `nft_approve` 调用
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> near_sdk::PromiseOrValue<String>;
}
```
