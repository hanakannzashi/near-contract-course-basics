# 第五章 单元测试与集成测试

## 单元测试
单元测试用于检测单元模块的代码逻辑, 从内部测试合约代码, 无法检测一些外部 API (如跨合约调用) 和宏 (如 `#[private]`)

如 [lib.rs](./src/lib.rs) 所示, 用 `#[cfg(test)]` 标注测试模块, 用 `#[test]` 标注测试任务, 测试主要使用 `near_sdk::test_utils` 模块

### 运行单元测试
使用 `cargo test` 命令运行单元测试

## 集成测试
集成测试需要使用 workspaces, 会在本地起一个 sandbox, 模拟链上环境, 可以进行外部测试. 有 [workspaces-rs](https://github.com/near/workspaces-rs) 和 [workspaces-js](https://github.com/near/workspaces-js) 两种版本, 本教程使用 js 版本

### 安装 workspaces-rs
```toml
[dependencies]
workspaces = "0.7.0"
```

### 安装 workspaces-js 
`yarn add near-workspaces -D`

### 安装 jest
workspaces 并不具备测试能力, 因此需要配合三方测试框架使用, 本教程使用 [jest](https://github.com/jestjs/jest) 作为测试框架 `yarn add jest @types/jest ts-jest -D`

在 [package.json](./package.json) 中配置 jest
```json
{
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "node",
    "moduleDirectories": [
      "node_modules"
    ]
  }
}
```

⚠️ jest 测试用例是并行的, 如果多个测试用例修改了同一个合约的状态, 可能会导致测试结果异常

### 运行集成测试
示例代码 [hello_test.spec.ts](./tests/hello_test.spec.ts).
使用 `yarn jest --testTimeout 60000 --detectOpenHandles` 命令运行集成测试, jest 会自动寻找项目根目录下所有结尾为 `.sepc.ts` 或 `.test.ts` 的文件并执行测试代码

## 运行测试用例
使用封装好的 `make test` 命令运行示例代码的单元测试用例和集成测试用例
