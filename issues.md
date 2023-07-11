/* cSpell:disable */
# Issues
## Current
- minesweeperpp demo

## Top
### Code
- Maximize test coverage
### Docs
- Clean up UML traits
- Indicate UML borrows
- Indicate UML generics
- Indicate methods and static fn

## Medium
- make UML naming uniform
- aggregate UML charts into one chart

## Backlog
- Implement patterns
  - Prototype
  - Singleton
  - Adapter
  - Bridge
  - Composite
  - Decorator
  - Facade
  - Flyweight
  - Proxy
  - Chain of Responsibility
  - Mediator
  - Momento
  - Strategy
  - Template Method

## Complete
- Move docs into .rs files
- Preview docs
- Unify docs
  - ensure svgs are referenced properly
  - branding reference works in rustdocs
- Visitor from proc macro
- Docs for primitives
  - attachable
  - emitter
  - detachable
  - executable
  - transformer
- Documentation on builder
- justfile dev loop parallelization cleanup
- Add UMLs to docs.
- UML for primitives
  - attachable
  - emitter
  - detachable
  - executable
  - transformer
- UML for aggregates
  - executable command
  - observing command
  - publisher subscriber
  - subscribing handler
- refactor builder
  - params struct in BuilderTokenStreamBuilder
  - split up BuilderTokenStreamBuilder::build
  - typestate in Builder pattern.
- Visitor macro breakup megamethod creating params
- embed-doc-image to dev-dependencies?


## Incoming
- Move to GATs
- builder <-> emitter
- parameterized docs in proc_macros using doc=
- builder Outputted type must impl `Builder`
- builder impl builder<i,o> & put on on proc_macro version
- builder investigate if builder should be emitter
- builder allow option types to be unset
- builder finish patternize
- builder Tests
- builder create test to ensure build fails as expected (on expect for popped values)
- visitor impl visitor<i,o> & put on on proc_macro version
- visitor Investigate the use of typestate/const enum/? in Builder pattern.
- visitor implements handler
- visitor add support for other types of enum structs
- visitor investigate adding deref on `#acceptor`
- submodules block diagram
- extends macro
- top level traits could be better as structs (simplifies boxing) - need investigation on this vs type aliasing, and manual boxing
- improve use of `delegate`.
- visitor add tuple (and one-ple) support
- investigate supporting easier transform/handler combinations.
  - eg: handler.push(transformer).handle(transformer_input)
- Improve docs for transformer handler
- Improve docs for derive pubsub
- Add builder borrow create to allow non-destructive build
- Introduce into_* fn's to easily translate between types
- Multi-transformer: Transformer of an array of transformers all taking the same type, then emitting the transformed values.  