physical

data 

network

transport

session 

presentation

application

**Addressing NEtworks**
mac addresses, 48 bit long unique id
  six groups of two hex digits
  can be edited by the os system

ip address
  cidr - 192.168.1.1/26 
    where 26 signifies the number of 1's in the network mask
    2^32-26 = 2^6 = 64 possible ip addresses
  assigned by DHCP in a home network

**how dns works**
dns server maintains mapping of human readable to ip address
dns server is /etc/resolv.conf
  this usually just points to the local dns server
  which then has prepopulated results, or knows who in the globe to ask

**connection models**
connection-oriented - where a virtual connection is negotiated before sending data.  That negotiation includes determining connection parameters.  This is what TCP is today, it consists of headers and data section.

connectionless - messages bear no relation to one another, example of UDP where there's no guarantee of sequence or reliability.


** RUST - OWNERSHIP **
exactly one owner at any point in time, others either have to borrow or copy that resource.  anything not owned will be dropped.

1. both resources are created on the stack
2. integer stored on the stack,

**rust - refernce**
```
example(&value) - passing the reference, no ownership change
. . .
fn example(&s)
```

**lifetimes**
conditions:
two inputs should have the same lifetimes
return values should have same lifetime as that of input

special lifetimes - {static, fn}

**generics**
