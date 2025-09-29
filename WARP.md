# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a bare-metal operating system kernel written in Rust, targeting x86_64 architecture. The project uses `#![no_std]` and `#![no_main]` attributes to create a freestanding binary that runs directly on hardware without the Rust standard library.

## Development Commands

### Prerequisites
This project requires Rust nightly toolchain (specified in `rust-toolchain` file) and the `bootimage` tool:
```bash
# Install bootimage tool (if not already installed)
cargo install bootimage
```

### Building
```bash
# Build the kernel
cargo build

# Build in release mode
cargo build --release

# The build uses a custom target specification (x86_64-os_rust.json)
# and automatically builds core/compiler_builtins from source
```

### Running
```bash
# Run in QEMU (uses bootimage runner configured in .cargo/config.toml)
cargo run

# Alternative: use the provided shell script
./emu.sh
```

### Testing
```bash
# Run all tests (unit tests and integration tests)
cargo test

# Run specific integration test
cargo test --test basic_boot

# Run the panic test (should_panic.rs)
cargo test --test should_panic
```

### Single Test Execution
```bash
# Run a specific test function within the main binary
cargo test -- --exact trivial_assert

# Run tests in a specific module
cargo test vga_buffer
```

## Code Architecture

### Core Components

**Kernel Entry Point (`src/main.rs`)**
- `_start()` function serves as the entry point (replaces standard main)
- Sets up panic handlers for both test and non-test builds
- Integrates with custom test framework for kernel testing

**Library Structure (`src/lib.rs`)**
- Exports core modules: `serial`, `vga_buffer`
- Implements custom test framework with QEMU exit codes
- Provides `Testable` trait for test function execution
- Handles test panic scenarios with proper QEMU exit codes

**VGA Buffer (`src/vga_buffer.rs`)**
- Direct memory-mapped I/O to VGA text buffer at address 0xb8000
- Thread-safe writer using `spin::Mutex` and `lazy_static`
- Custom `print!` and `println!` macros that replace standard library versions
- Implements scrolling, color support, and proper text formatting
- Uses `volatile` crate to prevent compiler optimizations on memory access

**Serial Communication (`src/serial.rs`)**
- UART 16550 serial port driver for debugging output
- Provides `serial_print!` and `serial_println!` macros
- Used primarily for test output and debugging information
- Communicates with host system through serial interface

### Custom Target Configuration

The project uses a custom target (`x86_64-os_rust.json`) with specific settings:
- No operating system (`"os": "none"`)
- Disabled red zone for interrupt safety
- Software floating point to avoid SSE/MMX instructions
- Uses `rust-lld` linker for bare metal linking

### Testing Framework

**Custom Test Runner**
- Bypasses standard library test framework
- Uses serial output for test results
- Implements proper QEMU exit codes for CI/CD integration
- Supports both unit tests and integration tests

**Integration Tests**
- `tests/basic_boot.rs`: Verifies basic kernel boot and println functionality
- `tests/should_panic.rs`: Tests panic handling behavior (expects failure)

### Memory Management

**Key Dependencies**
- `volatile`: Safe volatile memory access for hardware registers
- `spin`: Spinlock-based mutex for kernel synchronization  
- `lazy_static`: Static initialization with runtime initialization
- `x86_64`: Architecture-specific functionality and port I/O
- `uart_16550`: Serial port communication
- `bootloader`: Boot process handling

### Build Configuration

**Cargo Configuration (`.cargo/config.toml`)**
- Builds `core` and `compiler_builtins` from source
- Sets default target to custom x86_64-os_rust.json
- Configures bootimage as the runner for `cargo run`

**Bootimage Configuration**
- QEMU test arguments configured for headless testing
- Custom exit codes for test success/failure detection
- Debug exit device configuration for automated testing

## Development Notes

- All code must be `no_std` compatible
- Use `spin::Mutex` instead of `std::sync::Mutex` for synchronization
- Memory access to hardware should use `volatile` reads/writes
- Panic handlers must be defined for both test and non-test scenarios
- Serial output is preferred for debugging over VGA buffer output
- Tests run in QEMU and should exit with proper codes for automation
