# 第二章 合约的编译与部署

## 安装 near-cli
near-cli 是一个与 NEAR 区块链交互的终端工具, 有 [near-cli-rs](https://github.com/near/near-cli-rs) 和 [near-cli-js](https://github.com/near/near-cli) 两种版本.
其中 rs 版是交互式终端, 并且功能更加强大, 因此本教程选择 rs 版本

```shell
cargo install near-cli-rs
```

与 cli 交互
```shell
near
```

![near-cli-rs](./near-cli-rs.png)

### 切换 RPC
NEAR [官方 RPC](https://rpc.testnet.near.org) 需要科学上网, 因此我们可能需要使用个人 RPC, 可以通过 [infura](https://infura.io) 免费注册个人 RPC

以 macOS 为例, 获取测试网 RPC 后编辑 `~/Library/Application\ Support/near-cli/config.toml` 文件, 修改 `[network_connection.testnet]` 下的 `rpc_url` 字段即可切换 RPC

## 创建并导入 NEAR 账户
1. 打开测试网网页钱包 [MyNearWallet](https://testnet.mynearwallet.com), 根据指引注册 NEAR 账户, 保存好助记词
2. 将助记词导入终端
    ```shell
    near account import-account using-seed-phrase "${YOUR_SEED_PHRASE}" --seed-phrase-hd-path 'm/44'\''/397'\''/0'\''' network-config testnet
    ```

### Keychain
导入私钥时, 可以选择保存在 macOS keychain 或 legacy keychain 中

如果保存在 macOS keychain 中, 可以在 macOS 自带的**钥匙串访问**应用中找到私钥文件.
当导入同一个账户的多个不同私钥时, 虽然钥匙串名称是相同的, 但由于钥匙串账户不同, 私钥文件不会发生覆盖. 当需要签署交易的时候, 会自动去找钥匙串中可用的私钥进行签名.
**私钥文件不会被 iCloud 同步**
![macOS keychain](./macOS%20keychain.png)

如果保存在 legacy keychain 中, 可以在 `~/.near-credentials/${NETWORK_ID}` 目录中找到私钥文件. 包括一个与账户同名的 json 文件和一个与账户同名的目录,
目录里有一个与公钥同名的 json 文件, 该文件的内容和外面那个 json 是一样的, 都是私钥文件, 只是文件名不一样.
当导入同一个账户的多个不同私钥时, 最外面的 json 文件不会被覆盖, 而是将新的私钥文件保存在对应目录中. 当需要签署交易的时候, 会自动去找对应目录中可用的私钥进行签名
![legacy keychain](./legacy%20keychain.png)

## 编译第一章中的示例合约
1. 进入项目目录
    ```shell
    cd 1.\ 认识\ NEAR\ 智能合约
    ```
2. 安装 WASM 工具链
    ```shell
    rustup target add wasm32-unknown-unknown
    ```
3. 编译合约
    ```shell
    RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release
    ```
4. 将合约 WASM 文件移动到项目根目录下方便后续操作
    ```shell
    mkdir -p ./res && cp ./target/wasm32-unknown-unknown/release/hello_near.wasm ./res/
    ```

以上操作已经封装在 makefile 文件中
```shell
make all
```

## 部署和交互
假设你注册了两个测试网账户 `alice.testnet` 和 `code.testnet`, 一个用于作为主账户, 另一个用于作为合约账户, 私钥保存在 legacy keychain 中

1. 部署并初始化合约
    ```shell
    near contract deploy code.testnet use-file ./res/hello_near.wasm with-init-call init json-args '{"owner_id":"alice.testnet"}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
    ```
2. 调用 Change 方法
    ```shell
    near contract call-function as-transaction code.testnet set_account_description json-args '{"account_id":"bob.testnet","description":"Nice Bob"}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as alice.testnet network-config testnet sign-with-keychain send
    ```
3. 调用 View 方法
    ```shell
    near contract call-function as-read-only code.testnet get_account_description json-args '{"account_id":"bob.testnet"}' network-config testnet now
    ```
