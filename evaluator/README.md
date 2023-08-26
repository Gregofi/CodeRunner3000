# Evaluator
## Security

Since the code can be potentially malicious, a necessary measurements must be
taken to prevent the code from destroying our environment.
### Docker
#### CPU and Memory
The program is run with limited resources. A new container is spawned for every
program run with at least the following:
- `--pids-limit` - Limits the processes the container can spawn. The evaluator
should prevent from spawning new pids (for example `os.execute` is forbidden to
be used in the container), but in case the code find a way around it this acts like
a failsafe.
- `--cpuset-cpus` - Limits the cores the program can use.
- `--cpu-shares` - Lowers the priority of the container.
- `--memory` - RAM memory limit.

#### Filesystem
- The code is run in a container as unprivileged user.
- TODO: Limit users access to the internet.

### Language specific
Each language configuration must be done from ground up. The code runs in a docker container
for which Dockerfile must be provided. The evaluation is then left up to the configuration.
The source is often put into `/www/app/sources/<language>/<hash>`, which should be mounted
into the container. The evaluator then expects the output to be in the same folder under the
name `stdout.txt` and `stderr.txt`.

#### Dynamic Languages
In dynamic languages, we can freely redefine the code we want to forbid. For example
```lua
require = nil
```
