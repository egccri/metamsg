# Metamsg
> Low level network with custom layer that build custom protocol.

+ connection: Connection manager.
+ channel: Support reconnect, QoS etc.
+ routing: Device and network topology.
+ transmission: The pub/sub or rpc protocol that used for to communicate between devices.
+ discovery: Support discovery devices, find servers addr.
+ authmanager: AuthZ and AuthN.
+ adapter: Adapter OS.

### Notice

If you want to use auto discovery between devices, you need use metamsg private protocol implement your device, 
or use some protocol that support discovery, like coap, then bridge to other device by metamsg router.
 
### How to use metamsg

+ Use as a net lib like zenoh, zeromq, nanomsg, usually in public internet. 
+ Use as a computer operator system, provide auto discovery and data transfer, usually in local network.