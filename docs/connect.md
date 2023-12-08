# 设备连接与通信

##### 什么是连接

在Metamsg里，连接分为基于IP，以及非IP的，通过`connection`统一抽象，包括TCP、BLE、P2P等。但一般情况下我们不直接使用`connection`，
而是通过`channel`来在连接上收发数据，每一个`connection`都在系统中抽象为一个`channel`，在多个节点间通常只可以看到`channel`这一级，
关于`channel`，可以查阅`channel`章节。

##### 连接管理

连接由`Engine`管理。

##### 连接状态

每一个连接，会将状态更改回调给channel，channel会监听连接状态的改变以及错误信息，同时将状态更改反馈给routing，用于路由

##### 连接迁移


