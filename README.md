# Code Runner 3000
A multi-component monorepo hosting the necessary components for the
Code Runner 3000. It is a website allowing code execution. It is very similar to
[Compiler Explorer](https://godbolt.org/), offering languages that are (yet)
not available there. It does not aim to have disassembly like Compiler
Explorer.

## The architecture
There are be multiple components:
- website: A Flask website.
- evaluator: The evaluator of the code, a Rust API.
- dind: Responsible for spawning new docker images where the users code runs.
- db (todo): Storing persistent links to code.
