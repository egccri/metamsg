# Metamsg
> Low level network with custom layer that build custom protocol.

+ transmission: The pub/sub or rpc protocol that used for to communicate between devices.
+ adapter: Adapter transmission, and support basic utils to use common crates, include connection, router_link.
+ connection: Connection manager.
+ router_link: Device and network topology.
+ authmanager: AuthZ and AuthN.
+ discovery: Support discovery devices, find servers addr.

### Notice

If you want to use auto discovery between devices, you need use metamsg private protocol implement your device, 
or use some protocol that support discovery, like coap, then bridge to other device by metamsg router.
 
