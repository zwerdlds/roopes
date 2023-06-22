# Issues
## Current
- Fix references in docs to point to modules instead of directly to types.

## Top
- Maximize test coverage
- Clean up UML traits
- Indicate UML borrows
- Indicate UML generics
- Indicate methods and static fn's

## Medium
- Sort through backlog
- make UML naming uniform
- unite UML charts into one massive chart

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

## Incoming
- Unify generics ordering (IOT vs TIO)
- Generics naming conventions
- Investigate using GATs
- builder <-> emitter
- parameterized docs in proc_macros
- refactor builder
  - impl builder<i,o> & put on on proc_macro version
  - investigate if builder should be emitter
  - params struct in BuilderTokenStreamBuilder
  - split up BuilderTokenStreamBuilder::build
  - Investigate the use of typestate/const enum/? in Builder pattern.
    - I think a const enum with generics in the other slots could be an implementation option here.
  - Outputted type must impl `Builder`
  - Tests
- refactor visitor
  - impl visitor<i,o> & put on on proc_macro version
  - Investigate the use of typestate/const enum/? in Builder pattern.
  - Visitor implements handler
  - add support for other types of enum structs
  - investigate adding deref on `#acceptor`
- submodules block diagram
- extends macro
- top level traits could be better as structs (simplifies boxing) - need investigation on this vs type aliasing, and manual boxing
- improve use of `delegate`.
- Visitor macro breakup megamethod creating params
- visitor add tuple (and one-ple) support




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