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
On the other hand, optimizing for execution speed can conflict with the maintainability of a system.
Traits provided should give zero-cost-abstractions while possible.
However, working with v-tables has an inherent (though small) cost, so when it comes to the provided implementations, no guarantees about speed are provided.

It has also been observed that the use of `dyn` is inherently inefficient in Rust due to the inability for the compiler to see the indirected code in the client code, eliminating a good number of optimizations the compiler would otherwise be able to use on client code, probably resulting in less optimized builds.
`dyn` should occur in the provided traits, but implementations often use it (e.g: `Box`) or indirectly (e.g: `Vec`).

## Usage
To install, add the crate to your `cargo.toml` as usual.
The types provided are minimal, but the provided implementations should facilitate the most common uses.

`use ropes::prelude::*` will expose the essential traits.
Implementations are not exposed through `prelude` -- to use them, the specific implementation must be referenced in their module, such as `ropes::patterns::builder::Lambda`.
Users are suggested to use implementations in the following pattern to maintain hygienic namespace references.
Note in particular, `command::Lambda` is referable directly after including `prelude::*`:
``` rust
use ropes::prelude::*;
let command = command::Lambda::new(|| { println!("Hello world!"); });
command.execute();
```

# Patterns
Traits describing patterns are placed in one of three categories:

## Primitives
These form the basis of re-used abstractions used by patterns.
They can be used independently, but don't necessarily conform to a more widely-accepted pattern, so that may lead to undesirable qualities in your project if used directly.
[Please don't @ me.](https://en.wikipedia.org/wiki/Greenspun%27s_tenth_rule)
| Pattern                                              | Use                                                     | Namespace under `prelude` |
| :--------------------------------------------------- | :------------------------------------------------------ | :------------------------ |
| [`Attachable`](./ropes_primitives/src/attachable/)   | Provide an object to reference by another object.       | `attachable`              |
| [`Detachable`](./ropes_primitives/src/detachable/)   | Remove a previously added object from being referenced. | `detachable`              |
| [`Emitter`](./ropes_primitives/src/emitter/)         | Returns values.                                         | `emitter`                 |
| [`Executable`](./ropes_primitives/src/executable/)   | Obfuscates the execution some block of code.            | `executable`              |
| [`Handler`](./ropes_primitives/src/handler/)         | Consumes some value.                                    | `handler`                 |
| [`Transformer`](./ropes_primitives/src/transformer/) | Consumes and returns values.                            | `transformer`             |

## Patterns
The generally accepted, GoF-style patterns, most commonly used by developers.
| Pattern                                                                  | Use                                          | Namespace under `prelude` |
| :----------------------------------------------------------------------- | :------------------------------------------- | :------------------------ |
| [`Abstract Factory`](./ropes_lib/src/patterns/abstract_factory/)         | Abstracts creating objects.                  | `abstract_factory`        |
| [`Builder`](./ropes_derive/src/builder/)                                 | Aids in the construction of similar objects. | `builder`                 |
| [`Command`](./ropes_lib/src/patterns/command/)                           | Encapsulates a block of executable code.     | `command`                 |
| [`Heap Pool`](./ropes_lib/src/patterns/heap_pool/)                       | Reduces heap thrashing.                      | `heap_pool`               |
| [`Observer`](./ropes_lib/src/patterns/observer/)                         | Executes dynamic blocks of code.             | `observer`                |
| [`Publisher Subscriber`](./ropes_lib/src/patterns/publisher_subscriber/) | Sends messages to consuming blocks of code.  | `publisher_subscriber`    |
| [`State`](./ropes_lib/src/patterns/state/)                               | Alters its context's behavior dynamically.   | `state`                   |
| [`Visitor`](./ropes_lib/src/patterns/visitor/)                           | Type-based, multiple-object interactions.    | `visitor`                 |

## Aggregates
These patterns build on the common and primitive functions to provide bridges between patterns.
E.g: `Command` and the primitive `Executable` correspond closely, so a bridge struct which implements `Executable` for `Command` via a marker class forwards calls via a `Box<dyn Command>`.
These are provided to make the common case of moving between the given traits simpler, most often by calling `.into`.
| Pattern                                                                  | Use                                 | Namespace under `prelude` |
| :----------------------------------------------------------------------- | :---------------------------------- | :------------------------ |
| [`Executable Command`](./ropes_lib/src/aggregates/executable_command/)   | Adapts `Executable` from `Command`. | `executable_command`      |
| [`Observing Command`](./ropes_lib/src/aggregates/observing_command/)     | Adapts `Observer` from `Command`.   | `observing_command`       |
| [`Subscribing Handler`](./ropes_lib/src/aggregates/subscribing_handler/) | Adapts `Subscriber` from `Handler`. | `subscribing_handler`     |

# Examples
## [lambda-logger](./ropes_examples/lambda-logger/)
Demonstrates a stateful, functional-style logger system of a contrived logging system.

## [structuted-logger](./ropes_examples/structured-logger/)
Demonstrates a decoupled logging system.

# A Note on Issues
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