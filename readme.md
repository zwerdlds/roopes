![ropes project logo](promo/Logo.svg)

Ropes is a Rust Object Oriented Programming Element System.
This crate provides generic traits and implementations for typical object-oriented patterns in Rust.
It is intended to be used as a cluster of utility classes for implementing OOP-architected executables -- *in Rust!*.

## Goals
This package intends to meet the following criteria:

- Provide implementations of common OOP Design Patterns usable to compose larger programs.
- Document and implement reference implementations for students of OOP and Rust.
- Be easy to use for those familiar with the corresponding patterns.

## Optimization as a Non-Goal
It is convenient that Rust can produce low-level, optimized code.
On the other hand, optimizing for execution speed can often conflict with the maintainability of a system.
Traits provided should give zero-cost-abstractions while possible.
However, working with v-tables has an inherent cost, so when it comes to the provided implementations, no guarantees about speed are provided.

It has also been noted elsewhere that the use of `dyn` is inherently inefficient in Rust due to the inability for the compiler to see the  indirected code in the client code.
This eliminates a good number of optimizations the compiler would otherwise be able to use on client code, most likely resulting in less optimized builds.
`dyn` should not be used in the provided traits, but implementations often use it directly (e.g: `Box`) or indirectly (e.g: `Vec`).

## Usage
To install, add the crate to your `cargo.toml` as usual.
The types provided are minimal, but the provided implementations should facilitate the most common uses.

# Patterns
Traits describing patterns are placed in one of three categories:

## Primitives
These form the basis of re-used abstractions used by patterns.
They can be used independently, but don't necessarily conform to a [widely-accepted pattern], so that may lead to undesirable qualities in your project.
| Pattern                            | Notes                                        |
| :--------------------------------- | :------------------------------------------- |
| [`Attachable`](./src/primitives/)  |                                              |
| [`Detachable`](./src/primitives/)  |                                              |
| [`Emitter`](./src/primitives/)     | Returns values.                              |
| [`Executable`](./src/primitives/)  | Obfuscates the execution some block of code. |
| [`Handler`](./src/primitives/)     | Consumes some value.                         |
| [`Transformer`](./src/primitives/) | Consumes and returns values.                 |

## Patterns
The commonly accepted / GoF-style patterns, which are most commonly used by developers.
| Pattern                                                          | Notes |
| :--------------------------------------------------------------- | :---- |
| [`Abstract Factory`](./src/patterns/abstract_factory/)           |       |
| [`Builder`](./src/patterns/builder/)                             |       |
| [`Command`](./src/patterns/command/)                             |       |
| [`Heap Pool`](./src/patterns/heap_pool/)                         |       |
| [`Observer`](./src/patterns/observer/)                           |       |
| [`Publisher Subscriber`](./src/aggregates/publisher_subscriber/) |       |
| [`State`](./src/patterns/state/)                                 |       |

## Aggregates
These patterns build on the common and primitive functions to provide bridges between patterns.
E.g: `Command` and the primitive `Executable` correspond closely, so there is a bridge struct which implements `Executable` for `Command` via a marker class which indirects calls via a `Box<dyn ...>`.
These are provided to make the common case of moving between the given traits simpler.
| Pattern                                                        | Notes |
| :------------------------------------------------------------- | :---- |
| [`Executable Command`](./src/aggregates/executable_command/)   |       |
| [`Observing Command`](./src/aggregates/observing_command/)     |       |
| [`Subscribing Handler`](./src/aggregates/subscribing_handler/) |       |

# Examples
## [lambda-logger](./examples/lambda-logger/)
Demonstrates a stateful, functional-style logger system of a contrived logging system.

## [structuted-logger](./examples/structured-logger/)
Demonstrates a decoupled logging system.

# A Quick Note on Issues
Issues in this project are tracked with the system itself, not via an integrated tool, such as GitHub.
This enables issues to be tied to the repo, instead.
It may be beneficial to factor out issues into a separate repository for some independence, but necessitating a particular tool is unhealthy for the portability of this project.
[Issues are currently tracked here.](./issues.md)

# Dependencies
- `delegate`

This package is used to minimize boilerplate.
In particular, Rust does not have a trait inheritance system, so inheritance (where appropriate) needs to be implemented manually.
The `delegate!` macro enables these streamlined implementations.

- `enclose`

This package is used to simplify the process of copying reference-counted objects to and from lambdas.
