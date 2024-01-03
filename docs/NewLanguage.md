# Adding new language

You need to edit two components to add a new language. To understand the
format, the following context about how requests to the evaluator looks can
help. Requests to the evaluator are JSONs in the following format:

```json
{
    "name": "lua",
    "executor": "lua5.3.6",
    "code": "print(\"Hello, World!\")"
}
```

They may also be a `"compiler"` member.

## Evaluator

An example of configuration for the _Lua_ programming language:
```yaml
- name: lua
  executors:
  - name: lua5.4.6
  - name: lua5.3.6
  - name: lua5.2.4
  - name: lua5.1.5
  commands:
  - "${EXECUTOR} ${EXECUTOR_ARGS} ${SOURCE_FILE} ${SOURCE_ARGS}"
```

another example for _C_:

```yaml
- name: c
  compilers:
  - name: gcc-trunk
    path: /opt/evaluator/compilers/gcc-trunk/bin/gcc
  - name: gcc-bookworm
    path: /usr/bin/gcc
  commands:
  - cp ${SOURCE_FILE} source.c
  - ${COMPILER} ${COMPILER_ARGS} source.c
  - rm source.c
  - ./a.out
```

Fields:
- `name`: The identifying name of the action, the payload in requests will use this name.
- `commands`: Series of commands that will be used on the users code.
- `compilers` (optional): Supported compilers the evaluator can use.
    - `name`: Name of the compiler, the evaluator will match incoming payload
      against this name. If no executors matches, the server will return an
      error.
    - `path`: Path to the compiler. If not specified then the evaluator will
      search in `/opt/evaluator/compilers/<name>`
- `executors` (optional): Supported executors (interpreters) the evaluator can use.
    - Same fields and meaning as compiler.

The command has some values that will be substituted. Although they look like
environment variables, they are not and simple find and replace is performed
before the command is executed.
- `${SOURCE_FILE}` - Path to a source file which contains the users code from
  the payload. The file is not in current PWD and is read-only.
- ${SOURCE_ARGS}` - Command line arguments to the users code.
- `${EXECUTOR}` - The executor in the payload.
- `${EXECUTOR_ARGS}` - Arguments to the executor (for example to run `python3`
  with some additional flag).
- `${COMPILER}` and ${COMPILER_ARGS} - Same meaning as executor, but for
  `compiler`.

Configs are located in `/evaluator/config/config.yaml`.

### Getting the compiler/interpreter.

Some languages use the easy way and just install the compiler/interpreter via
`apt`. We do it with GCC, look at the example `gcc-bookworm`. If you want to
have specific version, then you would have to build it/download it, and put it
into the Evaluator docker image. We do this for Lua, you  can inspire yourself
from there. If you do install it by yourself, please try putting all necessary
code into `/opt/evaluator/compilers` (even if it is an interpreter).

Keep in mind that all commands run from jail (using `nsjail`). Sometimes, some
commands needs some folder or such that is not available, and it may take some
debugging to make it work. You can run the `nsjail` with production conf. by
doing `nsjail --config evaluator/config/userspace.cfg`. Edit the timelimit and
remove the logging line to ease debugging.

## Website

The website also has several places which will have to be edited.
In the `website/src/routes/code/+page.svelte`, there is an object containing
the definitions for each language.
- `name`: Used mainly for debugging and such.
- `server_name`: This will be sent as `name` in the payload.
- `editor_name`: The name that Monaco Editor uses for the language, see Monaco
  Editor documentation.
- `text`: What is displayed in the dropdown menu in the website.
- `executors`: Possible executors, should be the same as in the evaluator.

Then, add a new default program for the language in
`website/src/lib/defaultPrograms.ts`.
