# Architecture design

```
          Socket(Api)
        
     pub/sub   &&  req/resp
        
      Tree, Table, Channel
        
Routing, Session, Connection, Engine
```
+ Session是否需要
+ Socket持有Session和Connection，并且Session又可以自己运行, Session中可以自己运行的是Engine


#### Tree



#### 未知问题

1. 网络拓扑如何与协议协调