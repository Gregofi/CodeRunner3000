# Adding new language

## Evaluator
The evaluator runs the user code in a fresh docker container, which exist only
for the purpose of running users program. When the program finished, the container
and the corresponding volume is deleted.

Running a program for one language might require multiple steps to be taken.
For example compiled languages require to first compile the program and then to
execute it. Each of these steps are done in a fresh docker container. However,
to be able to persist some file between those, a directory
`/home/evaluator_nobody` is mounted onto a special volume which exists for all
steps. Files in this directory are carried over to the next stages. The
directory also contains the users code, which is in a file named `source`.

If you want the output of the command to be send back from the evaluator,
specify the items `stdout` and `stderr` to point to files `stdout.txt` and
`stderr.txt` respectively. Additional stages will always append to those files.
For each stage, specify a `name` (for logging) and a `command` which will run
in that stage. Also specify a `timeout` in seconds (default is five seconds).

As of now, all containers share the same packages. To install packages (for
example `gcc` for C) use the `packages` array in the top configuration.

The configs are in the `config.yaml` file, an example for C:
```yaml
- name: c
  run_options:
    memory_limit: 128m
    cpus_limit: 0.25
    pids_limit: 8
    storage_limit: 64m
  packages: ["gcc", "musl-dev"]
  steps:
    - name: "compile"
      command: ["gcc", "-O2", "-x", "c", "source", "-o", "/home/evaluator_nobody/source.exe"]
      timeout: 1
      stdout: "stdout.txt"
      stderr: "stderr.txt"
    - name: "execute"
      command: ["/home/evaluator_nobody/source.exe"]
      timeout: 5
      stdout: "stdout.txt"
      stderr: "stderr.txt"
```

## Website
The website needs to have a default value for editor `MonacoEditor.svelte` and
need to have specified names, which are in the `routes/code/+page.svelte`.
An example:
```javascript
{
    cpp23gcc: {
        name: 'cpp23gcc', // The name used for frontend logging
        server_name: 'cpp23gcc', // Send to the backend, should be same as `name` in the yaml file
        editor_name: 'cpp', // Used to pass to the monaco editor
        text: 'C++23 GCC' // Displayed name in the dropdown
    }
}
```
