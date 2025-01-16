# slab-allocator
Slab allocator implemented in Rust

Léo HAIDAR

## Summary

Table of contents
- [Introduction](#introduction)
- [Slab allocator](#slab-allocator)
- [Implementation](#implementation)
- [Usage](#usage)
- [Tests](#tests)
- [Benchmarks](#benchmarks)
- [License](#license)

## Introduction

This project is a slab allocator implemented in Rust. The slab allocator is a memory management technique that allows for efficient allocation and deallocation of memory blocks of fixed size. It is particularly useful in scenarios where the size of the memory blocks is known in advance and the allocation and deallocation of memory blocks is frequent.

## Slab allocator

The slab allocator is a memory management technique that allows for efficient allocation and deallocation of memory blocks of fixed size. It is particularly useful in scenarios where the size of the memory blocks is known in advance and the allocation and deallocation of memory blocks is frequent.

The slab allocator is composed of a set of slabs, each of which contains a set of memory blocks of the same size. The slab allocator maintains a list of slabs, each of which is associated with a specific size of memory blocks. When a memory block is requested, the slab allocator looks for a slab that contains free memory blocks of the requested size. If such a slab is found, the allocator returns a memory block from that slab. If no such slab is found, the allocator allocates a new slab and returns a memory block from that slab.

When a memory block is deallocated, the slab allocator returns the memory block to the slab from which it was allocated. If the slab becomes empty, the slab allocator deallocates the slab.

## Implementation

The slab allocator is implemented in Rust using the following data structures:
- `SlabAllocator`: The main slab allocator struct that contains a list of slabs.
- `Slab`: A slab struct that contains a set of memory blocks of the same size.
- `SlabBlock`: A memory block struct that contains a pointer to the memory block and a flag indicating whether the memory block is free or not.

The slab allocator provides the following methods:
- `new`: Creates a new slab allocator.
- `allocate`: Allocates a memory block of the specified size.
- `deallocate`: Deallocates a memory block.
- `stats`: Returns statistics about the slab allocator.

## Usage

To use the slab allocator, add the following dependency to your `Cargo.toml` file:

```toml
[package]
name = "slab-allocator"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = { version = "0.9.23", features = ["map_physical_memory"]}
volatile = "0.2.6"
spin = "0.5.2"
lazy_static = { version = "1.4.0", features = ["spin_no_std"] }
x86_64 = "0.14.2"

[package.metadata.bootimage]
test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04", "-serial", "stdio", "-display", "none"]
test-success-exit-code = 33
test-timeout = 300

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Then, you can use this command to build the project:

```bash
cargo +nightly bootimage
```

## Tests

To run the tests, use the following command:

```bash
cargo +nightly test
```

## Benchmarks

To run the benchmarks, use the following command:

```bash
cargo +nightly bench
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
