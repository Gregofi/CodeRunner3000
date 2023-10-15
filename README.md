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

## Running locally

When running the app locally, you can use the docker compose provided. However,
for developing the website, you probably want to develop it outside of a
container, to have hot-reloading and such.

If you do not need the backend code evaluator, you can just use `npm run dev`.
If you do, then do `docker compose up evaluator dind` and find its IP via
`docker inspect coderunner3000-evaluator-1`. This goes into `website/.env`.
