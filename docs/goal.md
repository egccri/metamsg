# Metamsg high-level design

### Goals

+ Support pub/sub model, support multi slave device protocol, like mqtt
+ Support virtual M2M channel that maybe bridge more than one router(node)
+ Support two sdk, consider slave device use metamsg or not use
+ If slave device use metamsg, then support req/resp model
+ Support bytes data transmission

### 场景

Metamsg 主要是为了支持 Egccri 的组网通行， Egccri 对于组网通行有如下要求：

+ 边缘设备自组网或直连 Egccri，匹配不同系统不同协议，Client 端包含自有 SDK、以及各种物联网操作系统
+ Egccri 与云端连接
+ Egccri 公网 P2P
+ Egccri 需要实现设备管理和连接管理

为了支持，Metamsg 需要有以下功能

+ 对于Client-SDK，可能需要裁减部分功能，并提供C代码，Client可以运行在各种国产新型操作系统上，考虑使用IPC
+ 局域网自发现自组网，Client支持广播消息，Egccri接收Client广播消息，或者Client知道Egccri地址，写死直连
+ 支持便捷的自定义协议，主要涉及编解码与连接管理
+ 对外暴露连接管理、设备管理、自定义协议

