# 跨合约调用与回调函数
NEAR 是一条异步链, 一笔交易会被拆分为一个或多个 Receipt 在不同的区块执行.
Receipt 在 NEAR SDK 中被抽象为 `Promsie`, 发起跨合约调用就是创建一个包含 `FunctionCall` 的 `Promise`, 调用逻辑在当前方法执行的区块不会执行, 而是在之后的区块中异步执行

由于 Receipt 跨了区块, 当交易失败时无法完全回滚, 只会回滚发生错误的那个 Receipt, 因此我们往往需要对调用结果进行回调, 以保证合约状态一致性

NEAR SDK 针对跨合约调用提供了高级 API, 该 API 本质上是对 Promise 的封装

## 跨合约调用示例 (使用高级 API)
该合约可以通过调用 [Linkdrop](https://github.com/near/near-linkdrop) 合约提供的 `create_account` 方法创建一个该合约账户的子账户, 用户需要支付一定的 NEAR 作为初始余额, 如果创建失败, 则退回这笔费用

1. 在 [cross.rs](./src/cross.rs) 中声明 Linkdrop 合约方法的接口
2. 在 [lib.rs](./src/lib.rs) 中实现回调函数 `resolve_create_account`, 主要实现账户创建失败后的退款逻辑
3. 在 [lib.rs](./src/lib.rs) 中编写完整的跨合约调用逻辑 `create_account_by_linkdrop`

## 手动分配 gas
跨合约调用时默认将剩余 gas 平均分配给所有调用, 高级 API 中可以通过 `with_static_gas` 和 `with_unused_gas_weight` 手动分配 gas
