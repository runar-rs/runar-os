# Runar OS

Runar OS is an experimental operating system written in Rust for RISC-V 64-bit systems.

The project is build form the ground up with a focus on a strongly typed kernel and idiomatic Rust rather than compatibility with existing operating-system interfaces.

> [!WARNING]
> Runar OS is an experimental project and is currently under development. It is not intended for production use.

## Overview
Runar is a hobby operating-system project written primarily in Rust. The current target architecture is RISC-V 64-bit (`riscv64gc-unknown-none-elf`).

## Design Goals
Runar is primarily an exploration of operating-system development, low-level programming, and the possibilities of building a modern operating system around Rust.

The project is guided by several overarching principles:

### Rust
Runar aims to make the best possible use of Rust's txpe system, ownership model, zero-cost abstractions, and standard library ecosystem. Rust should not merely be used as a safer
more colorful replacement for C, but should influence the architecture and interfaces of the operating system itself.

### Interoperability
Programs should be able to communicate and cooperate through well-defined, stable interfaces. Runar OS aims to make interaction between applications, system services, drivers and kernel
components straightforward.

### Explicit interfaces
Interfaces should be explicit and strongly types wherever possible. Abstractions should make the underlying behavior understandable.

### Documentation
Documentation is considered an integral part of the system rather than an afterthought. Every aspect of the project should be documented in a way that someone unfamiliar with
the project can understand it without having to reverse-engineer the implementations.

### Security
Security should be considered a fundermental property of the OS.

### Privacy
Privacy is considered a fundamental property of Runar OS. Users should have meaningful control over what information is collected, stored, processed and shared by their operating system.
Runar should follow the principle of data minimization: information should only be collected or retained when it is necessary for a specific and understandable purpose. Features should not
collect additional data simply becuase it may be useful in the future.

Runar OS should avoid unnecessary telemetry, tracking, profiling, and communication with external services. When information is transmitted outside the system, the purpose and destination should
be understandable to the user rather than hidden behind opaque background services.

Whenever reasonably possible, daata should be processed locally instead of being sent to centralized sercices. Users should be able to determine where their data is stroed and which components of
the system are able to access it.

Privacy should also be considered at the architectural level. System services and applications should operate with clearly defined permissions and access boundaries, limiting their ability to access
information that is unrelated to their purpose.

Runar should not treat privacy as a feature that users have to opt into. Privacy should be the default, while data sharing should be an explicit choice.
### Independence
Runar should remain as independent as reasonably possible from proprietary ecosystems, centralized services, and unnecessary external infrastructure. Users should be able to understand,
control, and modify the software running on their systems.
This principle also considers extreme scenarios in which external infrastructure may no longer be available, or not trustworthy. A system should not become unusable simply because a particular
company, online service, authentication provider, package repository or centralized infrastructure disappears. As a thought experiment, imagine a situation in which global communication
infrastructure is severly disrupted for an extended period of time. Runar should, where reasonably possible, remain a functional and maintainable system under such circumstances. Core
functionality should not depend on an external service merely because that service is convenient under normal conditions. This does **not** mean, that Runar OS must operate completely 
disconnected from the outside world. Networking, remote services and online resources are valuable capabilities. However those should remain **capabilities rather than single point of failure**.
This principle extends beyond disaster scenarios. It also applies to ordinary situations where services are discontinued, companies change their policies, netweorks become unavailable, or users
simply want to operate their systems independently.
