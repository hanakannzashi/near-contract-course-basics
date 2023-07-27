# 第三章 状态存储与容器

## 状态存储
NEAR 链上数据以 Key - Value 的形式存储，可以看做一个大的 Map

合约根结构（由 `#[near_bindgen]` 标记的结构体）会被序列化成一条记录，并且这条记录的 Key 固定为 `b"STATE"`, 对应的 base64 值为 `U1RBVEU=`，当合约被调用时，根结构会被反序列化
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAAAAAA="
  }
]
```

当我们仿照第一章中的示例合约，使用 `std::collections::HashMap` 作为容器时，这个容器包括它内部的数据都会作为合约根结构的一部分存在，当插入数据时，数据会被保存为根结构的一部分，使根结构变大
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAEAAAAFAAAAYWxpY2UEAAAAZ29vZA=="
  }
]
```

如果根结构一直增大，当合约被调用时反序列化需要消耗更多的 gas，可能会导致合约无法正常运行

当存储大量数据时，我们应该选择使用 `near_sdk::store` 或 `near_sdk::collections`，这些容器本身与它们内部的数据是分离的。当我们向容器中插入一条数据时，在链上会产生一条（或多条）新的记录，而不是将数据保存为合约根结构的一部分。当合约方法被调用时，这些数据默认不会被反序列化，只有当需要它们的时候才会反序列化
```json
[
  {
    "key": "U1RBVEU=",
    "value": "EgAAAGNvcm5mbG93ZXIudGVzdG5ldAAAAAA="
  },
  {
    "key": "DHYAAAAAAAAAAA==",
    "value": "HgAAAG1pbnQub3BlcmF0b3JzLm5hbWVza3kudGVzdG5ldA=="
  }
]
```

## 容器
near sdk 提供了两个容器模块，分别是 `near_sdk::store` 和 `near_sdk::collections` (以下简称 Store 和 Legacy Collections)。其中前者带有 cache，操作只有当合约方法执行完毕才会序列化到链上，后者每次操作都会立即序列化到链上

常用容器有
* `LookupMap ` 不可迭代的 Map
* `LookupSet` 不可迭代的 Set
* `Vector` 可迭代的 List
* `UnorderedMap` 可迭代的 Map，相比 `LookupMap` 存储占用更大
* `UnorderedSet` 可迭代的 Set，相比 `LookupSet` 存储占用更大
* `LazyOption` Lazy 数据类型，适合存储大型数据，如 metadata

由于每种容器在插入数据时都会产生一条新的链上记录，因此需要有唯一的前缀与容器绑定，用于区分不同容器的数据。因此这些容器在初始化时都需要传入唯一的 storage key 作为记录前缀

## 注意事项
Store 和 Legacy Collections 不仅是有没有 cache 的区别，在使用方法上也有区别。比如 Store 中的 `LookupMap` 的 `get_mut` 方法返回的 `Option<&mut V>` 是一个引用类型；而 Legacy Collections 中的 `LookupMap` 的 `get` 方法返回的 `Option<V>` 是一个所有权类型

这意味着如果对从容器中取出来的数据进行了修改，当使用 Legacy Collections 时，应该调用 `insert` 把修改结果重新保存；而当使用 Store 时则不需要额外处理
