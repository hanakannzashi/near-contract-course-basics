# 第零章 NEAR 核心概念
NEAR 是一条基于分片的异步区块链, 有一些与其他区块链不同的独特概念

## Block Time
NEAR 的出块时间在 1.2s 左右

## Finality
NEAR 的最终确认时间是 1s 左右

## Sharding
NEAR 是一条全分片区块链, 没有所谓的主链用于协调分片. 
验证节点只处理自己分片的交易, 也只打包自己分片的交易.
验证节点与分片的对应关系是隐藏的, 且会随周期改变.
跨分片通信至少需要 2 个区块才能完成

## Action
Action 是 NEAR 链上操作的基本单位

在 NEAR 上有且仅有以下 8 种 Action
* Transfer: 转账 NEAR
* FunctionCall: 合约调用
* CreateAccount: 创建子账户
* DeleteAccount: 删除账户
* AddKey: 新增公钥
* DeleteKey: 删除公钥
* Deploy: 部署合约
* Stake: 质押 NEAR

## Transaction
Transaction 是由用户私钥签名并支付 gas 费发起的一系列修改链上状态的 Action 的集合.

Transaction 包括三个关键参数
* signer id: 交易签名者
* receiver id: 交易接收者
* actions: Action 列表

## Receipt
Receipt 是为了处理跨分片通信而存在的概念.

Transaction 被打包进区块后不会直接执行, 而是先转换为 Receipt, 可以认为 Receipt 是一种可执行的 Transaction 格式.
Receipt 中的 Action 列表会在一个区块内执行完成, 执行具有原子性.

如果 Action 列表中包含 FunctionCall, 则执行完毕后可能产生新的子 Receipt.
子 Receipt 会在后面的区块执行, 因此子 Receipt 中的 Action 列表与父 Receipt 中的 Action 列表之间**不具有原子性**.
产生子 Receipt 是因为被调用的函数内部可能发生了跨合约调用 (FunctionCall Action) 或转账 (Transfer Action) 或其他操作, 这些操作都是异步的, 在函数被调用的那个区块不会执行

### Signer, Predecessor, Receiver
Signer 是绝对概念, Predecessor 和 Receiver 是相对概念.

假设用户 Alice 发起一笔交易调用了 ContractA 合约, ContractA 合约又调用了 ContractB 合约.
即 Alice -> ContractA -> ContractB.
Signer 始终是 Alice.
对于 Alice -> ContractA, Predecessor 是 Alice, Receiver 是 ContractA.
对于 ContractA -> ContractB, Predecessor 是 ContractA, Receiver 是 ContractB

## Account Model

### Named Account
NEAR 的账户是可读域名账户, 该特性由协议直接支持

没有 `.` 的账户是顶级账户, 其余都是子账户.
顶级账户 (32 字符以内) 只能由 [registrar](https://explorer.near.org/accounts/registrar) 创建, 该账户目前受 NEAR Foundation 控制.
子账户只能由其**直接父账户**创建, 子账户没有级数限制, 只要总长度不超过 64 个字符即可.
通过对公钥的 16 进制编码可以获得一个长度为 64 个字符的账户, 一般称为 Implicit Account, 其本质是一个顶级账户, 由于占满了 64 个字符, 它不能创建子账户

### Abstract Account
NEAR 的账户是抽象账户, 可以在指定账户直接部署智能合约, 该特性由协议直接支持

### Multi Key
NEAR 的账户采用多密钥对模型, 一个账户上面可以绑定多个公钥, 允许其对应的私钥访问该账户

密钥分为两种权限
* FullAccess: 完全访问权限
* FunctionCall: 只允许签署仅包含特定 FunctionCall Action 的交易

FunctionCall 权限的密钥通常在登陆 APP 的时候生成并给到 App 前端, 让 App 能自动帮用户签署低风险交易, 减少用户与钱包交互的频率, 从而优化用户的体验
