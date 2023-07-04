# Example gRPC Microservice

This is an example of a microservice written using Rust.

## Motivation
With clean code like this, who needs NodeJS?

## Dependencies
The project uses the following dependencies for various reasons:

| Dependency Name | Purpose                                    |
|-----------------|--------------------------------------------|
| Tokio           | Multithreading Support (built in to Tonic) |
| Tonic           | gRPC Server Implementation                 |
| Prost           | Protocol Buffers to gRPC Type Translation  |
| Diesel          | Database ORM and Query Builder             |

## Building the Project

**Prerequisites**
To build this project, one must have the Rust Toolchain installed.

Additionally, the library `libpq` must be installed on the system-side. On MacOS this can be installed via Homebrew:
```
brew install libpq
```

Failing to do so will result in a linker error.

**Cargo**
After all prerequisites are satisfied, the project can be built using Cargo:
```
cargo build
```

This will run a pre-build step, transpiling the file in `spec/account.proto` into
underlying Rust types using Prost. After this the server will begin running with Tonic,
implementing the underlying types provided by Prost.
