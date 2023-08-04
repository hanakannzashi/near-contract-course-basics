# 第五章 单元测试与集成测试

## 单元测试
单元测试用于检测单元模块的代码逻辑, 相对而言比较简单, 无法检测一些高级 API (如跨合约调用) 和宏 (如 `#[private]`)

如 [lib.rs](./src/lib.rs) 所示, 用 `#[cfg(test)]` 标注测试模块, 用 `#[test]` 标注测试任务, 测试主要使用 `near_sdk::test_utils` 模块


## 集成测试
集成测试会在本地起一个 sandbox, 模拟链上环境, 可以进行较完备的检测.

我们通过 workspaces 来启动 sandbox, 有 [workspaces-rs](https://github.com/near/workspaces-rs) 和 [workspaces-js](https://github.com/near/workspaces-js) 两种版本, 本教程使用 js 版本

安装 workspaces-rs
```toml
[dependencies]
workspaces = "0.7.0"
```

安装 workspaces-js `yarn add near-workspaces -D`
