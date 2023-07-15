# Evaluator
## Security
Since we are running potentially malicious code in our service, we must take
the necessary measurements to prevent from invading our system.
### Docker
#### CPU and Memory
We will run the program in a docker container which has limited resources
available. A new container will be spawned each time the program is run. We can
use the `--pids-limit` flag to prevent a fork bomb like attack. Next, the
`--cpuset-cpus` to limit the CPU cores the program will be able to use
(probably set it to one) and the `--cpu-shares` to make the container have less
of a priority in case other containers needs the computing power (like the
flask app). We must also limit the RAM memory, which is done simply via
`--memory`.

#### Filesystem
The application shouldn't be able to get out of the docker container. But just
in case, we will try to limit its access to the filesystem. We will use
`chroot` in the container to limit it to only certain tools and folders. Of
course, the program will be run with a user with as low priviledges as
possible. The docker file will also have restricted access to the internet.

### Language specific
#### Lua
Since Lua is a dynamic language, we can redefine its imports. We will be mainly
interested in commands line `popen`, file manipulations, require and so on.
```lua
require, os = function() print("Require is forbidden") end, nil
...
```
This has one unfortunate sideeffects, users errors will have invalid line
numbers when some errors happen. We could just require their code from some
main, which would make backtraces probably a little confusing but still better
than shifted lines.
