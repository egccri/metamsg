# Api

### 传输协议和通用Socket如何配合？

通过Metamsg可以内置常用的通信协议，而其他协议通过扩展内置协议实现，

在Zenoh中，通过先创建Session，然后Session创建一个Pub或者Sub，或者QueryTable对象。

### 多个外部协议如何统一连接管理？

```rust 
fn main() {
    // metamsg 需要的配置需要延后到transmission或者外部协议级别
    let metamsg = Metamsg::New();
    
    let socket = metamsg.trans(Pub/Sub);
    
    // 外部协议需要check transport，如何确保，因为MQTT支持BLE
    let mqtt = Metamsg::Mqtt(socket);
    
    let coap = Metamsg::Coap(socket);
    
    // 统一的连接管理以及追踪数据
    metamsg.manager();
}

```
### Router 和 Client 不同


