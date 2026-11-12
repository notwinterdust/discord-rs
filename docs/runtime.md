# Runtime notes

The project does not depend on an external runtime or logging crate.

`clients::TcpTransport` provides synchronous HTTP/1.1 over plain TCP. HTTPS and
WebSocket transports are supplied through the transport traits so applications
can choose a platform or standard-library-compatible implementation.

Gateway lifecycle code is synchronous and transport-driven. Applications own
the socket, scheduling, and TLS boundary while `GatewayConnection` handles
payload sequencing, heartbeats, reconnect decisions, and event dispatch.
