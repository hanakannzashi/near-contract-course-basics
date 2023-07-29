# 第二章 合约的编译与部署

## 安装 near-cli
near-cli 是一个与 NEAR 区块链交互的终端工具, 有 [near-cli-js](https://github.com/near/near-cli) 和 [near-cli-rs](https://github.com/near/near-cli-rs) 两种版本.
其中 RUST 版是交互式终端, 本教程使用 JavaScript 版

1. 安装 `yarn global add near-cli` 或 `cargo install near-cli-rs`. 两种 cli 的二进制命令都是 `near`, 如果你同时安装了两种 cli. 请设置 alias 加以区分

## 创建 NEAR 账户
1. 打开测试网网页钱包地址 [MyNearWallet](https://testnet.mynearwallet.com)
2. 根据指引注册 NEAR 账户, 保存好助记词 (测试网账户通常以 `.testnet` 结尾)
3. 将助记词导入终端 `near generate-key ${YOUR_ACCOUNT_ID} --seedPhrase="${YOUR_SEED_PHRASE}"`, 该命令会将你的助记词转换为私钥存储在 `~/.near-credentials/testnet` 目录下

## 编译第一章中的示例合约
1. 进入项目目录 `cd 1.\ 认识\ NEAR\ 智能合约`
2. 安装 WASM 工具链 `rustup target add wasm32-unknown-unknown`
3. 编译合约 `RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release`
4. 通常我们会将合约 WASM 文件移动到项目根目录下方便后续操作 `mkdir -p ./res && cp ./target/wasm32-unknown-unknown/release/hello_near.wasm ./res/`

以上操作全部封装在 makefile 文件中, 使用 `make build` 即可

## 部署合约
NEAR 可以将智能合约部署在指定账户, 无需像以太坊一样每次都部署在一个新的账户中
1. 假设你注册了两个测试网账户 `alice.testnet` 和 `code.testnet`, 一个用于作为主账户, 另一个用于作为合约账户
2. 部署合约 `near deploy code.testnet ./res/hello_near.wasm`
3. 初始化合约 `near call code.testnet init '{"owner_id":"alice.testnet"}' --account-id code.testnet`

## 与合约交互
* 调用 Change 方法 `near call code.testnet set_account_description '{"account_id":"bob.testnet","description":"Nice Bob"}' --account-id alice.testnet`
* 调用 View 方法 `near view code.testnet get_account_description '{"account_id":"bob.testnet"}'`
