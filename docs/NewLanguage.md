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

There may also be a `"compiler"` member.

## Evaluator

There are two parts of the evaluator that need to be edited.
First setups the compiler/interpreter.
Second tells the evaluator how to run the code.

### Infrastructure

The infra folder contains configs how to build the compilers and interpreters.
To add a new language add a folder with the name of the language in the `infra`.
In the folder, there should be a `setup.sh` script that will install it to the `/opt/evaluator/compilers/<name>` (interpreters too go into `compilers` ;).
Note that all dependencies must be installed in the same folder.
The final image will just copy this folder to the final image.
Look at the `lua` folder for an example.

Keep in mind that all commands run from jail (using `nsjail`).
Sometimes, some commands needs some folder or such that is not available, and it may take some debugging to make it work.
You can run the `nsjail` with production conf. by doing `nsjail --config evaluator/config/userspace.cfg`.
Edit the timelimit and remove the logging line to ease debugging.

After adding the folder, add a table to the `infra/languages/config.toml`.
The name of the table should be the name of the language.
Mandatory fields:
- `image`: The name of the image that will be used to build the language (for example lua uses image with gcc to compile the interpreter).

### Runtime

This tells the evaluator how to invoke the compiler or interpreter.

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

## Website

The website also has several places which will have to be edited.
In the `website/src/lib/constants.ts`, there is an object containing
the definitions for each language.
- `name`: Used mainly for debugging and such.
- `server_name`: This will be sent as `name` in the payload.
- `editor_name`: The name that Monaco Editor uses for the language, see Monaco
  Editor documentation.
- `text`: What is displayed in the dropdown menu in the website.
- `executors`: Possible executors, should be the same as in the evaluator.

Then, add a new default program for the language in
`website/src/lib/defaultPrograms.ts`.

### Syntax highlighting

Monaco editor provides default syntax highlighting for many languages.
Check if the language is supported [here](https://github.com/microsoft/monaco-editor/tree/main/src/basic-languages).

If not, add a syntax file to `website/src/lib/monaco/syntax-highlight`.
