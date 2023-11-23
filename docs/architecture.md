# Architecture design

```
          Socket(Api)
        
     pub/sub   &&  req/resp
     
        Context, Engine
        
      Tree, Table, Channel
        
  Discovery, Routing, Connection
```
+ Session是否需要
+ Socket持有Session和Connection，并且Session又可以自己运行, Session中可以自己运行的是Engine


#### Tree



#### 未知问题

1. 网络拓扑如何与协议协调？
    
   + req/resp
   + pub/sub
2. 寻址路由算法，两个可互通设备如何寻找到最短路径？

   设备只向路由节点发起请求，由路由作为第一跳寻找可以到达访问设备的路由，并确定最短路径