# Server Side

## src/tcp_echo_server.rs

Here we're making a server that echos back whatever you send through the stream:

```
$> rustc src/tcp_echo_server.rs && ./tcp_echo_server &
$> nc 127.0.0.1 8888
$> foo
foo
$> bar
bar
```

## src/tcp_client.rs

Conversely we can create a client that will send a message to the server, recieve the response (which is just being echoed) and then write that back to standard output.

Note: No `nc` needed:

```
$> rustc src/tcp_echo_server.rs && ./tcp_echo_server &
&> rustc src/tcp_client.rs && ./tcp_client
foo
foo
```