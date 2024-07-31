# Code Runner 3000
A multi-component monorepo hosting the necessary components for the
CodeRunner3000. It is a website allowing code execution, partially inspired by
[Compiler Explorer](https://godbolt.org/). It does however not aim to have disassembly like Compiler
Explorer.

## The architecture
There are be multiple components:
- Website = The frontend for the project.
- Website Proxy = A proxy to which all traffic goes before the website.
- Evaluator = Manages the logic behind spawning new jails in which the code runs.
- Statserver = Collects prometheus metrics from apps `/metrics` endpoint,
  aggregates them and serves them at its `/metrics` enpoint.

## Running locally

When running the app locally, you can use the docker compose provided. However,
for developing the website, you probably want to develop it outside of a
container, to have hot-reloading and such.

You can also run the evaluator locally,
but then you need to have the compilers and interpreters that you want to use match the config.
The address of the evaluator needs to be placed into `website/.env`,
if you want the website to connect to the evaluator. If you are okay with the code not being evaluated,
then you can just use `pnpm run dev`.

Alternatively, you can just run `docker compose up --build` and have it all without any work,
but it will be slower.

You might need to create new cgroup user named `NSJAIL`, or create folders in `<cgroup-folder>/pids/NSJAIL` and `<cgroup-folder>/cpu/NSJAIL`.

## Tests
Some of the components have unit tests, which can be run dependent on the technology
used in the component, for example Evaluator uses `cargo test`.

There are two versions of integration tests.
The first just tests the evaluator itself without the website.
It can be run like this:
```
docker compose -f compose.yaml -f integration_tests/evaluator.yaml run --build test
```

The second tests the whole system via playwright.
First, the system needs to be started:

```bash
docker compose up --build -d
```

Then the tests can be run:
```bash
pnpm run test:integration(-ui)
```

After the tests are done, the system can be stopped:
```bash
docker compose down
```

## Release
Create a new github release. The pipeline will build docker image for each component
and push it to dockerhub under the latest tag.
