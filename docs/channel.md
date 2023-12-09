# Channel

### 什么是Channel

`Channel`是`Connection`的封装，同时提供重试、超时、flight、消息匹配、路由匹配等功能，同时`Channel`上可以添加自定义Service。

### 路由

`Channel`在routing里抽象为client，参与路由，通过状态监听更改路由。