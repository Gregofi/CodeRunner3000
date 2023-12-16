# Code Runner 3000
A multi-component monorepo hosting the necessary components for the
CodeRunner3000. It is a website allowing code execution, being very similar to
[Compiler Explorer](https://godbolt.org/), offering languages that are (yet)
not available there. It does not aim to have disassembly like Compiler
Explorer.

## The architecture
There are be multiple components:
- Website = The frontend for the project.
- Website Proxy = A proxy to which all traffic goes before the website.
- Evaluator = Manages the logic behind spawning new containers in which code runs.
- statserver = Collects prometheus metrics from apps `/metrics` endpoint,
  aggregates them and serves them at its `/metrics` enpoint.

## Running locally

When running the app locally, you can use the docker compose provided. However,
for developing the website, you probably want to develop it outside of a
container, to have hot-reloading and such.

If you do not need the backend code evaluator, you can just use `npm run dev`.
If you do, then do `docker compose up evaluator dind` and find its IP via
`docker inspect coderunner3000-evaluator-1`. This goes into `website/.env`.

## Tests
Some of the components have unit tests, which can be run dependent on the technology
used in the component, for example Evaluator uses `cargo test`, while website might use
`pnpm run test`.

Integration tests are in separate folder, and can be run like this `docker
compose -f <test_file> run --build test`.

## Release
Create a new github release. The pipeline will build docker image for each component
and push it to dockerhub under the latest tag.
