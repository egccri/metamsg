# Pub/Sub协议设计

### 协议架构

+ 如果节点是router，配置了PubSub协议后，后台自动启动一个PubSub Broker,内部维护一个tree，元素为连接节点
+ 如果节点是client，可以配置Pub或者Sub，Pub可以是Pub一个或多个Device，同理

是否参考设计xpub/xsub，方便实现mqtt

https://250bpm.com/blog:39/index.html