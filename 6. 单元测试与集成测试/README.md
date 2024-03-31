# 单元测试与集成测试

## 单元测试
单元测试用于检测单元模块的代码逻辑, 从内部测试合约代码, 无法检测一些外部 API 和宏

如 [lib.rs](./src/lib.rs) 所示, 用 `#[cfg(test)]` 标注测试模块, 用 `#[test]` 标注测试任务, 测试主要使用 `near_sdk::test_utils` 模块

### 运行单元测试
运行单元测试
```shell
cargo test
```

## 集成测试
集成测试需要使用 workspaces, 会在本地起一个 sandbox, 模拟链上环境, 可以进行外部测试. 有 [workspaces-rs](https://github.com/near/workspaces-rs) 和 [workspaces-js](https://github.com/near/workspaces-js) 两种版本.
其中 JS 版本更加契合外部调用合约的习惯, 因此本教程选择 JS 版本

### 安装 workspaces
```shell
pnpm add near-workspaces -D
```

### 安装 jest
workspaces 并不具备测试能力, 因此需要配合测试框架使用, 本教程使用 [jest](https://github.com/jestjs/jest) 作为测试框架
```shell
pnpm add ts-jest jest @types/jest -D
```

在 [jest.config.json](./jest.config.json) 中配置 jest
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

### 运行集成测试
jest 会自动寻找后缀为 `.sepc.ts` 或 `.test.ts` 的文件并执行测试代码
```shell
pnpm test
```

## 运行示例测试代码
```shell
make all
```
