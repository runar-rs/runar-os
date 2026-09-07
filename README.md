# Runar OS

Runar OS is an experimental operating system written in Rust for RISC-V 64-bit systems. It is built
from the ground up around a strongly typed kernel and idiomatic Rust.

Now, you may be wondering why anyone would build an operating system from scratch.

An excellent question.

There are, after all, plenty of operating systems already. Some are even quite good.

And that, I suppose, is precisely the problem.

Runar is not intended to be another operating system which merely imitates the established ones
because that is what everyone expects an operating system to do.

Runar is an experiment in doing things differently.

It explores what happens when the architecture of an operating system is deliberately designed
around Rust, strong typing, explicit interfaces, open standards, and the rather radical notion that
the person who owns the computer ought to have some say in what it does.

A dangerous idea, perhaps.

But an interesting one.

> [!WARNING]
> Runar OS is an experimental project and is currently under development. It is not intended for
> production use. If you are looking for something boring, predictable and suitable for critical
> infrastructure, I am afraid this is probably not it.

## Overview
Runar is a hobby operating-system project written primarily in Rust. The current target architecture
is RISC-V 64-bit (`riscv64gc-unknown-none-elf`). 

RISC-V is a particularly fitting foundation for a project such as Runar OS. Its instruction-set
architecture is openly specified and designed to be freely implementable, providing a foundation
that can be studied without requiring permission from the keeper of some proprietary kingdom.

If we are going to build an operating system from the ground up, it seems only sensible to know what
sort of machinery lies underneath it.

Runar is primarily an exploration of operating system development, low-level programming, systems
architecture, and the possibilities of building a modern operating system around Rust.

It is not an attempt to conquer the world.

[Not yet anyway]: #


## Design Goals
Runar OS is guided by several overarching principles. They are not commandments, as those tend to be
rather inflexible.

Think of them instead as navigational charts: useful for knowing, where we are going, while still
leaving us enough freedom to discover that the map was wrong.

### Rust
Rust is not here merely to play the part of C with fewer opportunities to accidentally shoot
yourself in the foot. Runar OS aims to make proper use of Rust's type system, ownership model,
zero-cost abstractions and ecosystem.

The compiler may complain. It does that rather frequently, but a compiler refusing to compile 
questionable code is generally preferable to an operating system silently doing something
spectacularly wrong three hours after deployment.

### Interoperability
**Programs should be able to communicate and cooperate.**

A radical concept, I know.

Applications, system services, drivers and kernel components should interact through well-defined
and stable interfaces. A component should know what it may expect from another component, and what
is expected of itself in return.

Interfaces should not depend upon obscure implementation details, undocumented behavior, or some
ancient accident that nobody dares to remove because nobody remembers why it exists. While this 
kind of black magic can be tremendously useful (see `FastInvSqrt()`), it is not generally something
upon which one wants to base an operating system.

### Explicit interfaces
**Interfaces should be explicit and strongly typed wherever reasonably possible.**

An abstraction should make the behavior of a system easier to understand, not merely move the
confusing part somewhere else. There is, after all, a temptation in software engineering to hide
complexity beneath increasingly impressive layers of abstraction.

This can be useful, until nobody can remember what is actually happening.

Runar OS favors abstractions which reduce complexity without concealing the fundamental behavior of
the system.

The purpose of an abstraction is to make complexity manageable, not invisible.

### Open Source
Runar OS is open source.

This is not merely because it looks impressive on a project page. The source code should be
available for inspection, modification, and redistribution. If someone wants to know how Runar OS
works, they should be able to look. If they disagree with a design decision, they should be able to
inspect the reasoning. If they find a mistake, they should be able to fix it. If they have a better
idea, they should be able to build upon what is already there.

That is rather the advantage. Open source also means that Runar OS does not have to remain dependent
upon the people who originally wrote it.

### Documentation
Documentation is considered an integral part of the system rather than an afterthought. An operating
system is built from many layers of abstractions, assumptions, interfaces and hardware-specific
behavior, which seemed perfectly obvious at the time.

Leave those undocumented and eventually the knowledge exists only in the implementation or inside
the head of whoever wrote it.

Then that person leaves, and suddenly nobody knows why the strangest function in the kernel is
absolutely essential.
This is how ancient curses are created.

Runar OS should therefore document not only **what** a component does, but also **why** it does it.
Architectural decisions, interface contracts, invariants, hardware assumptions, limitations, and
safety requirements should be documented close to the code they describe.

The purpose is not to produce enormous quantities of documentation nobody reads. The purpose is to
preserve knowledge which would otherwise disappear.

Runar also maintains a separate documentation wiki defining the project's documentation standards,
because apparently the documentation needs documentation.
### Security
Security is a fundamental property of the operating system.

It should be considered during architectural design rather than added later as a collection of
patches after someone discovers that a particularly important door was never locked.

Security should influence permissions, isolation, interfaces, resource management and the overall
system architecture.

### Privacy
Privacy is a fundamental property of Runar OS.

Users should have meaningful control over what information is collected, stored, processed, and
shared by their operating system. Runar should follow the principle of data minimization. 
Information should only be collected or retained when it is necessary for a specific and
understandable purpose. If something is not needed, there is little reason to collect it. If it is
no longer needed, there is little reason to keep it. And if the answer is "It might be useful
someday", perhaps someday can collect its own data.

Runar should avoid unnecessary telemetry, tracking, profiling, and communication with external 
services. When information is transmitted outside the system, the purpose and destination should be
understandable to the user. No mysterious background communication. No unexplained data collection.
No assumption that the operating system is entitled to know everything simply because it happens to
be installed on the computer. Whenever reasonably possible, data should be processed locally rather
than sent to centralized services.

Users should be able to determine where their data is stored and which components are able to access
it. Privacy should also exist at the architectural level. System services and applications should
operate with clearly defined permissions and access boundaries.

A component should not have access to information simply because nobody thought to prevent it. And
privacy should not be something users have to discover hidden behind a collection of obscure
settings. Privacy should be the default.

Data sharing should be an explicit choice. The user's choice.
### Independence
Runar should remain as independent as reasonably possible from proprietary ecosystems, centralized
services, and unnecessary external infrastructure. Users should be able to understand, control, and
modify the software running on their systems. 

This is where the earlier principles begin to fit together. Open source allows the software to be
inspected and modified. RISC-V provides an openly specified instruction set architecture. Rust
provides strong tools for expressing correctness and ownership. Explicit interfaces make the
relationships between components understandable. Together, these things reduce the number of things,
which the user simply has to trust.

Trust is useful, but control is better.

Users should be able to understand, control, and modify the software running on their own systems.
They should not have to depend upon the continued goodwill of a particular company in order to
understand what their own computer is doing. Nor should the system become unusable merely because
some external service has disappeared. Imagine, for example, that a company shuts down a service, a
authentication provider disappears, a package repository becomes unavailable, a network connection
is lost; Or some corporation decides that the thing you have been using for ten years is now a
subscription.

What then?

Runar OS should, where reasonably possible, remain functional and maintainable. Core functionality
should not depend upon an external service merely because that service is convenient under normal
circumstances. Convenience is a wonderful thing until it isn't.

This does not mean Runar must be completely disconnected from the outside world. Networking is
useful, remote services are useful, online resources are useful. The internet is a splendid
invention. When it works. The important distinction is that these should remain capabilities rather
than single point of failure.

### Resilience

This principle also applies to less dramatic situations.

Services are discontinued, companies change their policies, networks become unavailable, software
gets abandoned, hardware becomes obsolete, people lose access to accounts and occasionally, somebody
decides that a perfectly functional piece of software should be replaced with a web application.

These things happen.

A system should be designed with the possibility that its surroundings may change. Runar should
therefore remain as self-contained and maintainable as reasonably possible. Its architecture should 
be understandable, its interfaces should be documented, its source should remain available and its
essential functionality should not require permission from some external authority.

Even in extreme circumstances, such as prolonged disruption of global communication infrastructure,
Runar should where reasonably possible remain something that can be operated, understood, and
maintained.
Not because the end of civilization is expected on Monday. It simply makes for a rather good test of
whether the system was unnecessarily dependent upon things outside itself.

## Conclusion

Taken together, these principles describe a rather simple idea.

Runar should be a system that its users can understand, a system they can inspect and that they can
modify. A system built upon open foundations rather than opaque dependencies.

Rust provides the language-level foundations. RISC-V provides an open architectural foundation. Open
source provides transparency and the possibility of continued development by people beyond the
original authors. Explicit interfaces provide understandable boundaries, security provides
protection, Privacy provides control over information and independence provides resilience.

None of this makes Runar easy to build. Quite the opposite, but easy was never really the point.
The interesting question is what happens when you take all of these ideas seriously and attempt to
build an operating system around them.

Perhaps it will work beautifully, perhaps it will fail spectacularly. Maybe, somewhere in between,
we will discover that one of the ideas was completely ridiculous.

That is rather what experiments are for.

And if Runar turns out to be useful along the way?

That would be a pleasant surprise.

But first, the operating system has to be developed.