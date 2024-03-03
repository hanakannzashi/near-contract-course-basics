# 认识 NEAR 智能合约
NEAR 是一条高性能分片 Layer1 公链. NEAR 账户是 NEAR 区块链的访问入口, 而智能合约就是部署在 NEAR 账户上的一段代码.
智能合约提供接口, 让外部能够访问和修改账户状态 (也叫合约状态). 目前, 一个账户只能部署一份合约代码, 重复部署会覆盖原有代码

通常的合约交互的流程: 发起合约调用 -> 执行合约代码 -> 读写合约状态 -> 返回调用结果.
流程中有 3 处主要的数据交互
1. 发起合约调用时的参数传递
2. 读写合约状态
3. 返回调用结果时的返回值传递

1 和 3 属于合约外部数据传递, 默认情况下使用 [json](https://github.com/serde-rs/json) 序列化.
2 属于合约内部数据传递, 默认情况下使用 [borsh](https://github.com/near/borsh-rs) 序列化

## 合约项目结构
NEAR 合约项目与普通的 Rust 项目结构一致
```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── lib.rs
```

[near-sdk-rs](https://github.com/near/near-sdk-rs) 是 NEAR 合约发开的最常用工具, 我们需要在 [Cargo.toml](./Cargo.toml) 中导入该库
```toml
[dependencies]
near-sdk = "4.1.1"
```

我们需要设置 `crate-type` 用于编译 WASM 二进制文件
```toml
[lib]
crate-type = ["cdylib"]
```

配置完成后下载依赖
```shell
cargo fetch
```

## 合约代码解析
[lib.rs](./src/lib.rs) 是合约代码的入口, 本教程编写了一个非常简单的智能合约, 该合约有一个所有者权限 `owner_id`, 合约存储账户 `AccountId` 以及账户对应的描述信息 `String`.
只有合约所有者可以修改描述信息，任何人都可以读取描述信息

### 合约根结构
`Contract` 结构体是合约的根结构, 使用 `#[near_bindgen]` 标记, 根结构需要被存储在合约状态里, 因此还需要实现 borsh 序列化

### 合约方法
合约方法被定义在使用 `#[near_bindgen]` 标记的 `impl` 块中, 并且只有 `pub` 的方法才能被合约外部访问

合约方法可以是
* 第一个参数为 `&self` 的 View 方法
* 第一个参数为 `&mut self` 的 Change 方法
* 第一个参数与 `self` 无关的合约初始化方法或 View 方法

### 合约初始化
合约默认使用 `Default` trait 来初始化, 因此我们必须为根结构实现这个 trait, 但是 `default` 方法没有参数, 用于初始化不太灵活, 绝大多数情况下我们都会自定义初始化方法.
`init` 方法就是一个带参数的初始化方法, 使用 `#[init]` 标记，同时我们还需要为根结构实现一个不可用的 `default` 方法 `#derive[PanicOnDefault]` 以通过编译.

### View 方法和 Change 方法
`get_account_description` 方法第一个参数是 `&self` 无法修改合约状态, 因此是一个 View 方法;
而 `set_account_description` 第一个参数是 `&mut self` 允许修改合约状态，因此是一个 Change 方法.
View 方法可以不消耗 gas 直接调用, 而 Change 方法必须发起一笔交易消耗 gas 去调用
