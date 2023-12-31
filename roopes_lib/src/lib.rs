//!
#![cfg_attr(feature = "doc-images",
  cfg_attr(all(),
    doc = ::embed_doc_image::embed_image!("roopes-logo", "src/logo.svg")
  ))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and \
           Rust version >= 1.54 to enable."
)]
#![deny(
    clippy::pedantic,
    clippy::style,
    clippy::correctness,
    clippy::complexity
)]
//! ![roopes logo][roopes-logo]
//!
//! Roopes is a Rust Object Oriented Pattern Element System.
//! This crate provides generic traits and implementations for typical
//! object-oriented patterns in Rust. It is intended to be used as a cluster of
//! utility classes for implementing OOP-architected executables -- *in
//! Rust!*.
//!
//! ## Goals
//! This package intends to meet the following criteria:
//! - Provide implementations of common OOP Design Patterns usable to compose
//!   larger programs.
//! - Document and implement reference implementations for students of OOP and
//!   Rust.
//! - Be easy to use for those familiar with the corresponding patterns.
//!
//! ## Optimization as a Non-Goal
//! It is convenient that Rust can produce low-level, optimized code.
//! On the other hand, optimizing for execution speed can conflict with the
//! maintainability of a system. Traits provided should give
//! zero-cost-abstractions while possible. However, working with v-tables has an
//! inherent (though small) cost, so when it comes to the provided
//! implementations, no guarantees about speed are provided.
//!
//! It has also been observed that the use of `dyn` is inherently less efficient
//! in Rust due to the inability for the compiler to see the indirected code in
//! the client code, eliminating a good number of optimizations the compiler
//! would otherwise be able to use on client code, probably resulting in less
//! optimized builds. `dyn` should occur in the provided traits, but
//! implementations often use it (e.g: `Box` or `Vec`).
//!
//! ## Usage
//! To install, add the
//! crate to your `cargo.toml` as usual. The types provided are minimal, but the
//! provided implementations should facilitate the most common uses.
//! `use roopes::prelude::*` will expose the essential traits. Implementations
//! are **not** exposed through `prelude` -- to use them, the
//! specific implementation must be referenced in their module, such as
//! `roopes::patterns::builder::Lambda`. To mitigate manually importing a large
//! number of implementations, roopes re-exports submodules which should make
//! the direct referencing of these types easier and more hygienic. In this
//! example, [`roopes_core::patterns::command::heap::Heap`] is re-used directly
//! from the [`prelude`] import.  E.g.:
//! ``` rust
//! use roopes::prelude::*;
//! let command = command::Heap::from(|| {
//!     {
//!         println!("Hello world!");
//!     }
//!     .into()
//! });
//! command.execute();
//! ```
//!
//! # Provided Patterns
//! Traits describing patterns are placed in one of three categories:
//! ## Primitives
//! These modules form the basis of re-used abstractions used by patterns.
//! These types exist to unify the syntax of the system: as a general rule, each
//! pattern contains a central group of traits made generic on some
//! user-specified type. e.g.: A builder is generic on the type on which
//! `build()` produces.
//! [Please don't @ me.](https://en.wikipedia.org/wiki/Greenspun%27s_tenth_rule)
//!
//! These types supply each of the following scenarios:
//!
//! | Type | Receives Value | Produces Value |
//! |--------|------------------|------------------|
//! | [`roopes_core::primitives::executable::Executable`] | No | No |
//! | [`roopes_core::primitives::emitter::Emitter`] | No | Yes |
//! | [`roopes_core::primitives::handler::Handler`] | Yes | No |
//! | [`roopes_core::primitives::transformer::Transformer`] | Yes | Yes |
//!
//! They can be used independently, but don't necessarily conform to a more
//! widely-accepted pattern other than various forms of `dyn Fn`, so that may
//! lead to undesirable qualities in your project if used directly.
//!
//! ## Patterns
//! The more generally accepted patterns.
//!
//! - [`roopes_core::patterns::abstract_factory`]
//! Defines a method of creating typed objects.
//! - [`roopes_core::patterns::command::Command`]
//! Encapsulates a block of executable code.
//! - [`roopes_core::patterns::heap_pool::HeapPool`]
//! Reduces heap thrashing.
//! - [`roopes_core::patterns::observer`]
//! Manages the distribution of notifications.
//! - [`roopes_core::patterns::publisher_subscriber`]
//! Dynamically receive messages.
//! - [`roopes_core::patterns::state`]
//! Manages a discreet-state algorithm.
//! - [`roopes_core::patterns::transformer_chain`]
//! Aids in creating multi-stage, type-safe, data transformations.
//! - [`roopes_derive::Builder`]
//! Aids in the configuration and construction of similar objects.
//! - [`roopes_derive::PubSub`]
//! Simplified subscriber dispatch type generation on a type.
//! - [`roopes_derive::Visitor`]
//! Ensures a type can consume a message type.
//!
//! ## Aggregates
//! These patterns build on the common and primitive
//! functions to provide bridges between patterns. E.g: `Command` and the
//! primitive `Executable` correspond closely, so a bridge struct which
//! implements `Executable` for `Command`. These are provided to make the case
//! of moving between the given traits simpler, most often by calling `.into`.
//!
//! - [`roopes_core::aggregates::command_executable`]
//! Adapts `Command` from `Executable`.
//! - [`roopes_core::aggregates::executable_command`]
//! Adapts `Executable` from `Command`.
//! - [`roopes_core::aggregates::executable_observer`]
//! Adapts `Executable` from `Observer`.
//! - [`roopes_core::aggregates::handling_publisher`]
//! Adapts `Handler` from `Publisher`.
//! - [`roopes_core::aggregates::observing_command`]
//! Adapts `Observer` from `Command`.
//! - [`roopes_core::aggregates::subscribing_handler`]
//! Adapts `Subscriber` from `Handler`.
//! - [`roopes_core::aggregates::transforming_handler`]
//! Adapts `Transformer` from `Handler`.
//!
//! # Examples
//! ## lambda-logger
//! Demonstrates a stateful, functional-style logger system of a contrived
//! logging system.
//!
//! ## structuted-logger
//! Demonstrates a decoupled logging system.
//!
//! ## collision-simulator
//! Demonstrates an enum-based visitor-based system.
//!
//! # A Note on Issues
//! Issues in this project are
//! tracked with the system itself, not via an integrated tool, such as GitHub.
//! This enables issues to be tied to the state of the repo.
//! It may be beneficial to factor out issues into a separate repository for
//! some independence, but necessitating a particular tool is unhealthy for the
//! portability of this project.
//! Issues are currently tracked in the `issues.md` in the root of the main
//! project.
//!
//! # Dependencies
//! - `delegate` used to minimize boilerplate.
//! In particular, Rust does not have a trait inheritance system, so inheritance
//! (where appropriate) needs to be implemented manually. The `delegate!` macro
//! enables these streamlined implementations
//! - `enclose` This package is
//! used to simplify the process of copying reference-counted objects to and
//! from lambdas.
//!
//! # Addenda
//! ## OOP in Rust?  Are you crazy!?
//! Maybe a little, but...
//! Once you accept the speed impact `Rc<...>` incurs,
//! especially if the client algorithms are organized for memory locality, it's
//! really not that bad. Client code should also try to organize by sub-system -
//! if the borrow checker is involved, it's a good idea to try and observer a
//! sort of system-level coherence. In essence: if code is large enough to
//! require OOP, it's probably at a point in its lifecycle where development
//! time is incurring more cost than runtime.
//!
//! ## Why tho?
//! Rust's type system is a good compromise between safety and usability.
//! Architecting large systems without patterns can lead to
//! difficult-to-maintain software. It is also that some patterns here will open
//! possibilities for bugs that could be avoided by more directly using Rust's
//! type-system. This library attempts compromise to leverage the patterns OOP
//! gives us, to enable larger projects, but also have Rust's type-system and
//! borrow checker on hand.
#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![allow(unused_imports)]
#[macro_use]
extern crate roopes_derive;
extern crate roopes_core;

/// The building blocks for patterns and aggregates.
pub mod primitives
{
    pub use roopes_core::primitives::*;
}

/// Object oriented patterns.
pub mod patterns
{
    pub use roopes_core::patterns::*;
    pub use roopes_derive::*;
}

/// Methods of transmuting between patterns.
pub mod aggregates
{
    pub use roopes_core::aggregates::*;
}

/// Exposes the most used types from the library.
pub mod prelude
{
    pub use roopes_core::prelude::*;
    pub use roopes_derive::*;
}
