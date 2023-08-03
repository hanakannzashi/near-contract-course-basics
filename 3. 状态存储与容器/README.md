# 第三章 状态存储与容器

## 状态存储
NEAR 链上数据以 Key - Value 的形式存储

合约根结构 (由 `#[near_bindgen]` 标记的结构体) 会被序列化成一条记录, 并且这条记录的 Key 固定为 `b"STATE"`, 对应的 base64 值为 `U1RBVEU=`.
当合约被调用时, 根结构会被反序列化到内存

### 合约状态
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAAAAAA="
  }
]
```
初始化后的合约状态只有一条记录, 其中 `value` 的值就是我们定义的 `Contract` 结构体

### 插入数据后的合约状态
当我们仿照第一章中的示例合约, 使用 `std::collections::HashMap` 作为容器时, 这个容器包括它内部的数据都会作为合约根结构的一部分存在.
当插入数据时, 数据会被保存为根结构的一部分, 使根结构变大
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAEAAAAFAAAAYWxpY2UEAAAAZ29vZA=="
  }
]
```

可以看到 `value` 对应的字符串变长了但是记录的数量没改变

## 容器
如果根结构一直增大, 当合约被调用时反序列化需要消耗更多的 gas, 一旦超出 gas 上限可能会导致合约无法正常运行

为了解决这个问题, near sdk 提供了两个容器模块, 分别是 `near_sdk::store` 和 `near_sdk::collections` (以下简称 Store 和 Legacy Collections).
这些容器本身与它们内部的数据是分离的, 当我们向容器中插入一条数据时, 在链上会产生一条 (或多条) 新的记录, 而不是将数据保存为合约根结构的一部分.

⚠️ Store 和 Legacy Collection 的容器均未实现 json 序列化, 因此默认情况下无法把它们作为合约方法的返回值, 如果希望返回容器内的全部数据, 请使用迭代器功能转换为 `std::collections`️

### 使用 Store 或 Legacy Collection 时合约状态示例
插入数据前
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAEAAAAA"
  }
]
```

插入数据后
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAEAAAAA"
  },
  {
    "key": "AAUAAABhbGljZQ==",
    "value": "BAAAAGdvb2Q="
  }
]
```

可以看到合约根结构的 `value` 值并没有变化, 但是产生了一条新的记录, 这些记录只有在需要的时候才会参与反序列化

### 常用容器
* `LookupMap ` 不可迭代的映射类型
* `LookupSet` 不可迭代的集合类型
* `Vector` 可迭代的列表类型
* `UnorderedMap` 可迭代的映射类型, 相比 `LookupMap` 存储占用更大
* `UnorderedSet` 可迭代的集合类型, 相比 `LookupSet` 存储占用更大
* `LazyOption` 按需加载的数据类型, 适合存储大型数据, 如 metadata

由于每个容器在插入数据时都会在全局状态中产生一条新的记录, 需要有唯一的前缀与容器绑定, 用于区分不同容器的数据.
因此这些容器在初始化时都需要传入唯一的 storage key 作为记录前缀

可以使用 rust 的 `enum` 类型配合 `BorshStorageKey` 宏得到 storage key

### Store 与 Legacy Collection 的区别
|               | Store                                | Legacy Collection |
|---------------|--------------------------------------|-------------------|
| 缓存            | 有                                    | 无                 |
| 插入的数据类型       | 所有权类型 `K` 和 `V`                      | 引用类型 `&K` 和 `&V`  |
| 获取的数据类型       | 引用类型 `Option<&V>` 或 `Option<&mut V>` | 所有权类型 `Option<V>` |
| 修改数据后是否需要重新插入 | 不需要                                  | 需要                |

Store 与 Legacy Collection 在使用方式上区别较大, 通常不建议在同一个项目中同时使用这两个模块, 建议使用 Store
