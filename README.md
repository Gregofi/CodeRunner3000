# Code Runner 3000
A multi-component monorepo hosting the necessary components for the CodeRunner3000.
It is a website allowing code execution, partially inspired by [Compiler Explorer](https://godbolt.org/).
It does however not aim to have disassembly like Compiler Explorer.

## The architecture
There are be multiple components:
- Website = Svelte website, what the user faces.
- Proxy = Nginx server to which all traffic goes before the website. Adds headers, compression etc.
- Evaluator = Rust API which executes the code.
- Statserver = Aggregates metrics from the microservices and serves them.

## Running locally

When running the app locally, you can use the docker compose provided.
However, for developing the website, you probably want to develop it outside of a container,
to have hot-reloading and such.
To do this, follow the instructions in the website folder.

To run all the components, you need to have docker installed.
Please do note that the evaluator runs as privileged, since it needs to edit cgroups.
You can use the following command:

```bash
make up
# or
docker compose up --build
```

You might need to create new cgroup user named `NSJAIL`,
or create folders in `<cgroup-folder>/pids/NSJAIL` and `<cgroup-folder>/cpu/NSJAIL`.

### Infrastructure

The evaluator builds the compilers and interpreters.
This can take a while, we do it on every push to master.
If you want to build it yourself, you can use the following command:

```bash
make build
```

However, be prepared to wait for a long while.
You may however need to build just a few languages, for example when adding a new one.
To do this, you can use the following command in the `evaluator/infra` folder:

```bash
make build LANGUAGES="rust python"
```

This will build only the rust and python compilers and interpreters.
With this, be prepared that your tests will probably fail since you are missing some compilers.
When you are done, you can pull the image from dockerhub again with

```bash
make pull
```

## Tests
Some of the components have unit tests, which can be run dependent on the technology
used in the component, for example Evaluator uses `cargo test`.

There are two versions of integration tests.
The first just tests the evaluator itself without the website.
It can be run like this:

```bash
make test
# OR
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

### CI

The tests also run as part of the CI pipeline.
They must pass in order to merge to master.
They use the ghcr.io images for compilers, unless you changed `evaluator/infra`,
in which case the compilers will be built as part of the pipeline.

The E2E tests always use the latest images from ghcr.
This is intended, do not put language specific tests in the E2E tests.
They should be in the evaluator tests.

## Release
Create a new github release.
The pipeline will build docker image for each component and push it to dockerhub under the latest tag.
