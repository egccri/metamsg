# Architecture design

```
          Socket(Api)
        
   pub/sub && req/resp && mqtt
     
        Context, Engine
        
      Tree, Table, Channel
        
  Discovery, Routing, Connection
```
+ Session是否需要
+ Socket持有Session和Connection，并且Session又可以自己运行, Session中可以自己运行的是Engine

### 消息（数据）流

### 控制流


#### Tree



#### 未知问题

1. 网络拓扑如何与协议协调？
   新建连接时加入router，确定通信时寻找路径，并在本地记录路径
   Channel 如何表示远端
   Router 有一个id和scope，client-id 需要用户来确定，/scope/router_id/client-id 确定一个端点
   client需要加入网络拓扑，参与寻址
   除了/scope/router_id/client-id唯一，其他的在MetamsgContext下唯一，比如channel-id
   如果通信，必需确定目标的client-id，或者使用pub/sub协议来根据topic（group）通信，而在路由层，
   一个client的路由node_id为router的node_id:client_id,client的node_id抽象为虚拟节点参与广播，
   并与一个channel绑定。
2. client迁移后，router中的设备相关数据通过p2p也需要迁移，同时应用迁移。
3. 寻址路由算法，两个可互通设备如何寻找到最短路径？

   设备只向路由节点发起请求，由路由作为第一跳寻找可以到达访问设备的路由，并确定最短路径
4. router 间消息中转，也需要支持pubsub模式和query模式，同时需要实现多路复用