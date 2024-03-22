# grpc-demo

This is a simple gRPC demo project written in Rust. It contains a simple gRPC server and a client.

The codebase is the specific implementation of the [tutorial](https://konghq.com/blog/engineering/building-grpc-apis-with-rust), very well written by [Shane Utt](https://github.com/shaneutt).

It's still three tasks to be done:

- adding [TLS and auth](https://grpc.io/docs/guides/auth/) to the client and server to protect data

- add command line flags for the cli and server, enable changing `host:port`

- improve the appearance of the CLI output for better human readability
