# Echo Server

This makes use of the standard library's built in `std::net::TcpListener`, to run `cd echo_server` and `cargo run`, then from another terminal you can open connections to port 8888:

```
$> #nc (or netcat) can establish tcp/udp connections
$> nc 127.0.0.1 8888
$> foo
foo
$> bar
bar
```